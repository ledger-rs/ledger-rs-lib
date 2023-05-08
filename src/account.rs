use std::cell::RefCell;

use crate::post::Post;

/**
 * Account
 */

#[derive(Debug, PartialEq)]
pub struct Account {
    // parent
    pub name: String,
    // note
    // depth
    pub accounts: Vec<Account>,
    // pub posts: Vec<Post>,
    // deferred posts
    // value_expr
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            accounts: vec![],
        }
        // posts: vec![]
    }
}
