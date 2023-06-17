/*!
 * Account definition and operations
 */

use std::{collections::HashMap, vec, rc::Rc, cell::RefCell};

use crate::{
    balance::Balance,
    journal::{AccountIndex, Journal, PostIndex}, post::Post,
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
    pub posts: Vec<Rc<RefCell<Post>>>,
    // indices of Posts in the Journal.Posts array.
    // pub post_indices: Vec<PostIndex>,
    // deferred posts
    // value_expr
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            parent_index: None,
            name: name.to_owned(),
            accounts: HashMap::new(),
            // post_indices: vec![],
            posts: vec![],
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
    pub fn amount(&self, journal: &Journal) -> Balance {
        let mut bal = Balance::new();

        // for index in &self.post_indices {
        //     let post = journal.get_post(*index);
        //     bal.add(&post.amount.unwrap());
        // }
        for post in &self.posts {
            let amount = post.borrow().amount.unwrap();
            bal.add(&amount);
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
        total += self.amount(journal);

        total
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{amount::Decimal, journal::Journal, parse_file, parser};

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

    #[test]
    fn test_amount() {
        let mut journal = Journal::new();
        parse_file("tests/basic.ledger", &mut journal);
        let index = journal.find_account_index("Assets:Cash").unwrap();
        let account = journal.get_account(index);

        let actual = account.amount(&journal);

        assert!(!actual.amounts.is_empty());
        assert_eq!(Decimal::from(-20), actual.amounts[0].quantity);
        let commodity = journal.get_amount_commodity(actual.amounts[0]).unwrap();
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
            journal
                .commodity_pool
                .get_commodity(actual.amounts[0].commodity_index.unwrap())
        );

        assert_eq!(actual.amounts[0].quantity, (-30).into());
        assert_eq!(actual.amounts[0].commodity_index, Some(0.into()));
    }
}
