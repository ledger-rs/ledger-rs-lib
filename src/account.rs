/*!
 * Account definition and operations
 */

use std::{collections::HashMap, vec};

use crate::{
    balance::Balance,
    journal::{AccountIndex, Journal}, post::Post,
};

#[derive(Debug, PartialEq)]
pub struct Account {
    // parent
    pub parent_index: Option<AccountIndex>,
    pub name: String,
    // note
    // depth
    pub accounts: HashMap<String, AccountIndex>,
    // pub posts: Vec<Post>,
    /// indices of Posts in the Journal.Posts array.
    // pub post_indices: Vec<PostIndex>,
    posts: Vec<*const Post>,
    // deferred posts
    // value_expr
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            parent_index: None,
            name: name.to_owned(),
            accounts: HashMap::new(),
            posts: vec![],
            // post_indices: vec![],
        }
    }

    pub fn fullname(&self, journal: &Journal) -> String {
        let mut parent_index_opt = self.parent_index;
        let mut fullname = self.name.to_owned();

        while parent_index_opt.is_some() {
            let acct = journal.get_account(parent_index_opt.unwrap());
            parent_index_opt = acct.parent_index;
            if !acct.name.is_empty() {
                fullname = format!("{}:{}", acct.name, fullname);
            }
        }

        fullname
    }

    pub fn get_account(&self, name: &str) -> Option<AccountIndex> {
        Some(*self.accounts.get(name).unwrap())
    }

    /// Returns the amount of this account only.
    pub fn amount(&self) -> Balance {
        let mut bal = Balance::new();

        for post_ptr in &self.posts {
            let post: Post;
            unsafe {
                post = post_ptr.read();                
            }
            bal.add(&post.amount.unwrap());
        }

        bal
    }

    /// Returns the balance of this account and all sub-accounts.
    pub fn total(&self, journal: &Journal) -> Balance {
        let mut total = Balance::new();

        // Sort the accounts by name
        let mut acct_names: Vec<_> = self.accounts.keys().collect();
        acct_names.sort();

        // iterate through children and get their totals
        for acct_name in acct_names {
            let index = self.accounts.get(acct_name).unwrap();
            let subacct = journal.get_account(*index);
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

    #[test]
    fn test_fullname() {
        let mut j = Journal::new();
        let input = r#"2023-05-01 Test
    Expenses:Food  10 EUR
    Assets:Cash
"#;
        parser::read_into_journal(Cursor::new(input), &mut j);

        let Some(acct_id) = j.find_account_index("Expenses:Food")
            else {panic!("account not found");};
        let account = j.get_account(acct_id);

        let actual = account.fullname(&j);

        assert_eq!(5, j.accounts.len());
        assert_eq!("Food", account.name);
        assert_eq!("Expenses:Food", actual);
    }

    /// Test parsing of Amount
    #[test]
    fn test_amount_parsing() {
        let mut journal = Journal::new();
        parse_file("tests/basic.ledger", &mut journal);
        let index = journal.find_account_index("Assets:Cash").unwrap();
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
