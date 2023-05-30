use std::{collections::HashMap, vec};

use crate::journal::{AccountIndex, PostIndex, Journal};

/**
 * Account
 */

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

    pub fn find_account(&mut self, name: &str, journal: &mut Journal) -> Option<AccountIndex> {

        // if not found
        let acct = Account::new(name);
        let index = journal.add_account(acct);

        self.accounts.insert(name.to_owned(), index);

        Some(index)
    }

    pub fn parse(input: &str) -> Self {
        let input = input.trim();

        if input.is_empty() {
            panic!("Invalid account")
        }

        // Self { parent_index: None, name: input.to_string(), accounts: vec![], post_indices: vec }
        Self::new(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::journal::Journal;

    use super::Account;

    #[test]
    fn test_parse_simple() {
        let input = "Assets";

        let actual = Account::parse(input);

        assert_eq!(input, actual.name);
    }

    #[test]
    fn poc_for_account_tree() {
        let mut j = Journal::new();
        let assets_id = j.add_account(Account::new("Assets"));
        // add to master
        {
            let master = j.get_master_account_mut();
            master.accounts.insert("Assets".into(), assets_id);
        }
        let master = j.get_master_account();

        // Assert
        assert_eq!(2, j.accounts.len());
        assert_eq!("Assets", j.accounts.get(assets_id).unwrap().name);

        assert_eq!(1, master.accounts.len());
    }
}
