use crate::{amount::Amount, xact::Xact};

/**
 * Posting
 */

pub struct Post {
    // pub xact: Option<&'a Xact<'a>>,

    pub account: String,
    pub amount: Option<Amount>,
}

impl Post {
    pub fn new(account: &str, amount: Option<Amount>) -> Self {
        Self {
            // xact: None,
            
            account: account.to_string(),
            amount,
        }
    }

    pub fn empty() -> Self {
        Self { account: "".to_string(), amount: Some(Amount::null()) }
    }
}
