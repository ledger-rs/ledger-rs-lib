use crate::{amount::Amount, account::Account};

/**
 * Posting
 */

 #[derive(Debug, PartialEq)]
pub struct Post {
    /// Pointer to the Account.
    pub account: usize,
    /// Pointer to the Xact.
    pub xact: usize,

    // TODO: remove this temp field
    pub account_temp: Account,
    pub amount: Amount,
}

impl Post {
    pub fn new(account: &str, amount: Option<Amount>) -> Self {
        Self {
            account: usize::MAX,
            xact: usize::MAX,

            account_temp: Account::new(account),
            amount: Amount::null(),
        }
    }

    /// Creates a Post from post tokens.
    pub fn create(tokens: [&str; 2]) -> Self {
        // parse account?
        // parse amount
        
        todo!()
    }

    pub fn empty() -> Self {
        Self::new("", None)
    }
}
