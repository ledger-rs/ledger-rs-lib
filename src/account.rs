/**
 * Account
 */

pub struct Account {
    // parent
    name: String,
    // note
    // depth
    accounts: Vec<Account>,
    // posts
    // deferred posts
    // value_expr
}

impl Account {
    pub fn new(name: String) -> Self {
        Self { name, accounts: vec![] }
    }
}