use crate::{amount::Amount, account::Account};

/**
 * Posting
 */

 #[derive(Debug, PartialEq)]
pub struct Post {
    // pub xact: Option<&'a Xact<'a>>,
    pub xact_index: Option<usize>,

    pub account: Account,
    pub amount: Option<Amount>,
}

impl Post {
    pub fn new(account: &str, amount: Option<Amount>) -> Self {
        Self {
            // xact: None,
            xact_index: None,
            
            account: Account::new(account),
            amount,
        }
    }

    pub fn empty() -> Self {
        Self::new("", None)
    }
}
