/*!
 * Account
 */

use std::{collections::HashMap, vec};

use crate::journal::{AccountIndex, Journal, PostIndex};

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
    pub post_indices: Vec<PostIndex>,
    // deferred posts
    // value_expr
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            parent_index: None,
            name: name.to_owned(),
            accounts: HashMap::new(),
            post_indices: vec![],
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

    pub fn amount(&self, journal: &Journal) {
        todo!()
    }

    pub fn total(&self, journal: &Journal) {
        // let total;

        // iterate through children and get their totals
        for (name, index) in &self.accounts {
            let subacct = journal.get_account(*index);
            let subtotal = subacct.total(journal);
            todo!("add to total");
        }

        // let amount = 
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{journal::Journal, parser, parse_file};

    use super::Account;

    #[test]
    fn test_fullname() {
        let mut j = Journal::new();
        let input = r#"2023-05-01 Test
    Expenses:Food  10 EUR
    Assets:Cash
"#;
        parser::read_into_journal(Cursor::new(input), &mut j);

        let Some(acct_id) = j.create_sub_account(0, "Expenses:Food", false)
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
        let index = journal.find_account("Assets:Cash").unwrap();
        let account = journal.get_account(index);
        let actual = account.amount(&journal);

        todo!("assert")
    }

    // #[test]
    fn test_total() {
        let acct = Account::new("Cash");
        
        // let actual = acct.total();

        todo!()
    }
}
