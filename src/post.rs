use crate::{amount::Amount, xact::Xact, account::Account};

/**
 * Posting
 */

 #[derive(Debug, PartialEq)]
pub struct Post {
    // pub xact: Option<&'a Xact<'a>>,

    pub account: Account,
    pub amount: Option<Amount>,
}

impl Post {
    pub fn new(account: &str, amount: Option<Amount>) -> Self {
        Self {
            // xact: None,
            
            account: Account::new(account),
            amount,
        }
    }

    pub fn empty() -> Self {
        Self::new("", None)
    }
}
