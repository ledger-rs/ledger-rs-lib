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
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            parent: std::ptr::null(),
            name: name.to_owned(),
            accounts: HashMap::new(),
            posts: vec![],
            // post_indices: vec![],
        }
    }

    pub fn fullname(&self) -> String {
        // let mut parent_index_opt = self.parent_index;
        let mut parent: *const Account = self.parent;
        let mut fullname = self.name.to_owned();

        while !parent.is_null() {
            // let acct = journal.get_account(parent_index_opt.unwrap());
            // let acct = journal.get_account(parent);
            let acct: &Account = self.get_account(parent);

            parent = acct.parent;
            if !acct.name.is_empty() {
                fullname = format!("{}:{}", acct.name, fullname);
            }
        }

        fullname
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

            let mut new_account = Account::new(name);
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{amount::Quantity, journal::Journal, parse_file, parser};

    /// Search for an account by the full account name.
    #[test]
    fn test_fullname() {
        let mut j = Journal::new();
        let input = r#"2023-05-01 Test
    Expenses:Food  10 EUR
    Assets:Cash
"#;
        parser::read_into_journal(Cursor::new(input), &mut j);

        let Some(acct_id) = j.find_account("Expenses:Food")
            else {panic!("account not found");};
        let account = j.get_account(acct_id);

        let actual = account.fullname();

        assert_eq!(5, j.accounts.len());
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
        let assets = journal.find_account("Assets").unwrap();

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
