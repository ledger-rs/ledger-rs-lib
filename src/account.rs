/**
 * Account
 */

#[derive(Debug, PartialEq)]
pub struct Account {
    // parent
    pub parent_index: Option<usize>,
    pub name: String,
    // note
    // depth
    pub accounts: Vec<Account>,
    // pub posts: Vec<Post>,
    /// indices of Posts in the Journal.Posts array.
    pub post_indices: Vec<usize>,
    // deferred posts
    // value_expr
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            parent_index: None,
            name: name.to_owned(),
            accounts: vec![],
            post_indices: vec![],
        }
    }
}
