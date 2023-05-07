use crate::{amount::Amount, xact::Xact};

/**
 * Posting
 */

pub struct Post {
    // pub xact: Option<&'a Xact<'a>>,

    pub account: String,
    pub amount: Amount,
}

impl Post {
    pub fn new() -> Self {
        Self {
            // xact: None,
            
            amount: Amount::new(),
            account: "".to_string(),
        }
    }
}
