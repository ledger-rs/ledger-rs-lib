/*!
 * Account definition and operations
 */

use std::{collections::HashMap, ptr::addr_of, vec};

use crate::{balance::Balance, post::Post};

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

    /// called from find_or_create.
    fn create_account(&self, first: &str) -> &Account {
        let mut new_account = Account::new(first);
        new_account.set_parent(self);

        let self_mut = self.get_account_mut(self as *const Account as *mut Account);

        self_mut.accounts.insert(first.into(), new_account);

        let Some(new_ref) = self.accounts.get(first)
            else {panic!("should not happen")};

        log::debug!("The new account {:?} reference: {:p}", new_ref.name, new_ref);
        new_ref
    }

    pub fn fullname(&self) -> &str {
        // skip the master account.
        if self.parent.is_null() {
            return "";
        }

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
        // let ptr = self as *const Account;
        let ptr = addr_of!(*self);
        let subject = self.get_account_mut(ptr);

        subject.fullname = fullname;
    }

    /// Finds account by full name.
    /// i.e. "Assets:Cash"
    pub fn find_account(&self, name: &str) -> Option<&Account> {
        if let Some(ptr) = self.find_or_create(name, false) {
            let acct = Account::from_ptr(ptr);
            return Some(acct);
        } else {
            return None;
        }
    }

    /// The variant with all the parameters.
    /// account_t * find_account(const string& name, bool auto_create = true);
    pub fn find_or_create(&self, name: &str, auto_create: bool) -> Option<*const Account> {
        // search for direct hit.
        if let Some(found) = self.accounts.get(name) {
            return Some(found);
        }

        // otherwise search for name parts in between the `:`

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

        if let Some(account_opt) = self.accounts.get(first) {
            // keep this value
            account = account_opt;
        } else {
            if !auto_create {
                return None;
            }

            account = self.create_account(first);
        }

        // Search recursively.
        if !rest.is_empty() {
            let acct = self.get_account_mut(account);
            account = acct.find_or_create(rest, auto_create).unwrap();
        }

        Some(account)
    }

    pub fn from_ptr<'a>(acct_ptr: *const Account) -> &'a Account {
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
            let post: &Post;
            unsafe {
                post = &**post_ptr;
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
        // Confirms the pointers are the same:
        // assert_eq!(parent as *const Account, addr_of!(*parent));
        // self.parent = parent as *const Account;

        self.parent = addr_of!(*parent);
        
        // log::debug!("Setting the {:?} parent to {:?}, {:p}", self.name, parent.name, self.parent);
    }

    /// Returns the balance of this account and all sub-accounts.
    pub fn total(&self) -> Balance {
        let mut total = Balance::new();

        // Sort the accounts by name
        let mut acct_names: Vec<_> = self.accounts.keys().collect();
        acct_names.sort();

        // iterate through children and get their totals
        for acct_name in acct_names {
            let subacct = self.accounts.get(acct_name).unwrap();
            // let subacct = journal.get_account(*index);
            let subtotal = subacct.total();

            total += subtotal;
        }

        // Add the balance of this account
        total += self.amount();

        total
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, ptr::addr_of};

    use crate::{amount::Quantity, journal::Journal, parse_file, parse_text, parser};

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
        for _a in j.master.flatten_account_tree() {
            //println!("sub-account: {:?}", a);
            counter += 1;
        }

        assert_eq!(3, counter);
    }

    /// Search for an account by the full account name.
    #[test]
    fn test_fullname() {
        let input = r#"2023-05-01 Test
    Expenses:Food  10 EUR
    Assets:Cash
"#;
        let mut journal = Journal::new();
        parser::read_into_journal(Cursor::new(input), &mut journal);

        let account = journal.find_account("Expenses:Food").unwrap();

        let actual = account.fullname();

        assert_eq!(5, journal.master.flatten_account_tree().len());
        assert_eq!("Food", account.name);
        assert_eq!("Expenses:Food", actual);
    }

    /// Test parsing of Amount
    #[test]
    fn test_amount_parsing() {
        let mut journal = Journal::new();

        // act
        parse_file("tests/basic.ledger", &mut journal);

        let ptr = journal.find_account("Assets:Cash").unwrap();
        let account = journal.get_account(ptr);

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
        let actual = assets.total();

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

    #[test]
    fn test_parent_pointer() {
        let input = r#"2023-05-05 Payee
    Expenses  20
    Assets
"#;
        let mut journal = Journal::new();

        // act
        parse_text(input, &mut journal);

        let ptr = journal.master.find_account("Assets").unwrap();
        let assets = journal.get_account(ptr);

        assert_eq!(addr_of!(journal.master), assets.parent);
    }

    #[test]
    fn test_parent_pointer_after_fullname() {
        let input = r#"2023-05-05 Payee
    Expenses  20
    Assets
"#;
        let mut journal = Journal::new();
        parse_text(input, &mut journal);

        // test parent
        let ptr = journal.master.find_account("Assets").unwrap();
        let assets = journal.get_account(ptr);

        assert_eq!(&journal.master as *const Account, assets.parent);

        // test fullname
        let assets_fullname = journal.master.accounts.get("Assets").unwrap().fullname();
        let expenses_fullname = journal.master.accounts.get("Expenses").unwrap().fullname();

        assert_eq!("Assets", assets_fullname);
        assert_eq!("Expenses", expenses_fullname);

        // test parent
        let ptr = journal.master.find_account("Assets").unwrap();
        let assets = journal.get_account(ptr);

        assert_eq!(addr_of!(journal.master), assets.parent);
    }

    #[test_log::test]
    fn test_parent_pointers() {
        let input = r#"2023-05-05 Payee
        Expenses:Groceries  20
        Assets:Cash
    "#;
        let mut journal = Journal::new();

        parse_text(input, &mut journal);

        // expenses
        let expenses = journal.master.find_account("Expenses").unwrap();
        assert_eq!(addr_of!(journal.master), expenses.parent);

        // groceries
        let groceries = expenses.find_account("Groceries").unwrap();
        assert_eq!(expenses as *const Account, groceries.parent);

        // assets
        let assets = journal.master.find_account("Assets").unwrap();
        assert_eq!(addr_of!(journal.master), assets.parent);

        // confirm that addr_of! and `as *const` are the same.
        assert_eq!(assets as *const Account, addr_of!(*assets));

        // cash
        let cash = assets.find_account("Cash").unwrap();
        assert_eq!(assets as *const Account, cash.parent);

    }
}
