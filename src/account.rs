/*!
 * Account definition and operations
 */

use std::{collections::HashMap, vec};

use crate::{balance::Balance, journal::Journal, post::Post};

#[derive(Debug, PartialEq)]
pub struct Account {
    pub(crate) parent: *const Account,
    pub name: String,
    // note
    // depth
    pub accounts: HashMap<String, Account>,
    pub posts: Vec<*const Post>,
    // deferred posts
    // value_expr
    fullname: String,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            parent: std::ptr::null(),
            name: name.to_owned(),
            // note
            accounts: HashMap::new(),
            posts: vec![],
            fullname: "".to_string(),
            // post_indices: vec![],
        }
    }

    pub fn fullname(&self) -> &str {
        if !self.fullname.is_empty() {
            return &self.fullname;
        }

        let mut fullname = self.name.to_owned();
        let mut first = self;

        while !first.parent.is_null() {
            // If there is a parent account, use it.
            first = self.get_account_mut(first.parent);

            if !first.name.is_empty() {
                fullname = format!("{}:{}", &first.name, fullname);
            }
        }

        self.set_fullname(fullname);

        &self.fullname
    }

    fn set_fullname(&self, fullname: String) {
        // alchemy?
        let ptr = self as *const Account;
        let mut_ptr = ptr as *mut Account;
        let subject = self.get_account_mut(mut_ptr);

        subject.fullname = fullname;
    }

    /// Finds account by full name.
    /// i.e. "Assets:Cash"
    pub fn find_account(&mut self, name: &str) -> Option<*const Account> {
        self.find_or_create(name, true)
    }

    /// The variant with all the parameters.
    /// account_t * find_account(const string& name, bool auto_create = true);
    fn find_or_create(&mut self, name: &str, auto_create: bool) -> Option<*const Account> {
        if let Some(found) = self.accounts.get(name) {
            return Some(found);
        }

        // otherwise create

        let mut account: *const Account;
        let first: &str;
        let rest: &str;
        if let Some(separator_index) = name.find(':') {
            // Contains separators
            first = &name[..separator_index];
            rest = &name[separator_index + 1..];
        } else {
            // take all
            first = name;
            rest = "";
        }

        if let Some(account_opt) = self.accounts.get_mut(first) {
            // keep this value
            account = account_opt;
        } else {
            if !auto_create {
                return None;
            }

            let mut new_account = Account::new(first);
            new_account.set_parent(self);

            self.accounts.insert(first.into(), new_account);

            let Some(new_ref) = self.accounts.get(first)
                else {panic!("should not happen")};
            account = new_ref;
        }

        // Search recursively.
        if !rest.is_empty() {
            let acct = self.get_account_mut(account);
            account = acct.find_or_create(rest, auto_create).unwrap();
        }

        Some(account)
    }

    pub fn get_account(&self, acct_ptr: *const Account) -> &Account {
        unsafe { &*acct_ptr }
    }

    pub fn get_account_mut(&self, acct_ptr: *const Account) -> &mut Account {
        let mut_ptr = acct_ptr as *mut Account;
        unsafe { &mut *mut_ptr }
    }

    pub fn flatten_account_tree(&self) -> Vec<&Account> {
        let mut list: Vec<&Account> = vec![];
        self.flatten(&mut list);
        list
    }

    /// Returns the amount of this account only.
    pub fn amount(&self) -> Balance {
        let mut bal = Balance::new();

        for post_ptr in &self.posts {
            let post: Post;
            unsafe {
                post = post_ptr.read();
            }
            if let Some(amt) = post.amount {
                bal.add(&amt);
            }
        }

        bal
    }

    fn flatten<'a>(&'a self, nodes: &mut Vec<&'a Account>) {
        // Push the current node to the Vec
        nodes.push(self);
        // If the node has children, recursively call flatten on them
        for (_name, child) in &self.accounts {
            child.flatten(nodes);
        }
    }

    pub(crate) fn set_parent(&mut self, parent: &Account) {
        self.parent = parent;
    }

    /// Returns the balance of this account and all sub-accounts.
    pub fn total(&self, journal: &Journal) -> Balance {
        let mut total = Balance::new();

        // Sort the accounts by name
        let mut acct_names: Vec<_> = self.accounts.keys().collect();
        acct_names.sort();

        // iterate through children and get their totals
        for acct_name in acct_names {
            let subacct = self.accounts.get(acct_name).unwrap();
            // let subacct = journal.get_account(*index);
            let subtotal = subacct.total(journal);

            total += subtotal;
        }

        // Add the balance of this account
        total += self.amount();

        total
    }
}

// impl <'a>IntoIterator for &'a Account {
//     type Item = &'a Account;
//     type IntoIter = AccountIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         let mut nodes: Vec<&Account> = vec![];
//         self.flatten(&mut nodes);

//         AccountIterator {
//             // inner: self.accounts.values()
//             inner: nodes
//         }
//     }
// }

// pub struct AccountIterator<'a> {
//     // inner: std::collections::hash_map::Values<'a, String, Account>,
//     inner: Vec<&'a Account>
// }

// impl Iterator for AccountIterator<'a> {
//     type Item = &Account;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.iter().next()
//     }
// }

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{amount::Quantity, journal::Journal, parse_file, parser};

    use super::Account;

    #[test]
    fn test_flatten() {
        let mut j = Journal::new();
        let _acct = j.register_account("Assets:Cash");
        let mut nodes: Vec<&Account> = vec![];

        j.master.flatten(&mut nodes);

        assert_eq!(3, nodes.len());
    }

    #[test]
    fn test_account_iterator() {
        let mut j = Journal::new();
        let mut counter: u8 = 0;

        let _acct = j.register_account("Assets:Cash");
        for a in j.master.flatten_account_tree() {
            //println!("sub-account: {:?}", a);
            counter += 1;
        }

        assert_eq!(3, counter);
    }

    /// Search for an account by the full account name.
    #[test]
    fn test_fullname() {
        let mut j = Journal::new();
        let input = r#"2023-05-01 Test
    Expenses:Food  10 EUR
    Assets:Cash
"#;
        parser::read_into_journal(Cursor::new(input), &mut j);

        let Some(ptr) = j.find_account("Expenses:Food")
            else {panic!("account not found");};
        let account = j.get_account(ptr);

        let actual = account.fullname();

        assert_eq!(5, j.master.flatten_account_tree().len());
        assert_eq!("Food", account.name);
        assert_eq!("Expenses:Food", actual);
    }

    /// Test parsing of Amount
    #[test]
    fn test_amount_parsing() {
        let mut journal = Journal::new();

        // act
        parse_file("tests/basic.ledger", &mut journal);

        let index = journal.find_account("Assets:Cash").unwrap();
        let account = journal.get_account(index);

        let actual = account.amount();

        // assert
        assert!(!actual.amounts.is_empty());
        assert_eq!(Quantity::from(-20), actual.amounts[0].quantity);
        let commodity = actual.amounts[0].get_commodity().unwrap();
        assert_eq!("EUR", commodity.symbol);
    }

    /// Test calculation of the account totals.
    #[test_log::test]
    fn test_total() {
        let mut journal = Journal::new();
        parse_file("tests/two-xact-sub-acct.ledger", &mut journal);
        let ptr = journal.find_account("Assets").unwrap();
        let assets = journal.get_account(ptr);

        // act
        let actual = assets.total(&journal);

        // assert
        assert_eq!(1, actual.amounts.len());
        log::debug!(
            "Amount 1: {:?}, {:?}",
            actual.amounts[0],
            actual.amounts[0].get_commodity()
        );

        assert_eq!(actual.amounts[0].quantity, (-30).into());
        assert_eq!(actual.amounts[0].get_commodity().unwrap().symbol, "EUR");
    }
}
