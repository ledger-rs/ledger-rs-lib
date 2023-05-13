use std::vec;

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

    pub fn parse(input: &str) -> Self {
        if input.is_empty() {
            panic!("Invalid account")
        }

        // Self { parent_index: None, name: input.to_string(), accounts: vec![], post_indices: vec }
        Self::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn test_parse_simple() {
        let input = "Assets";

        let actual = Account::parse(input);

        assert_eq!(input, actual.name);
    }

    #[test]
    fn test_parse_parent() {
        let input = "Expenses:Dining";

        let actual = Account::parse(input);

        assert_eq!("Dining", actual.name);

        todo!("assert account hierarchy")
    }

}