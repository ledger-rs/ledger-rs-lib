use crate::amount::Amount;

/**
 * Posting
 */

pub struct Post {
    pub account: String,
    pub amount: Amount,
}

impl Post {
    pub fn new() -> Self {
        Self { amount: Amount::new(), account: "".to_string() }
    }
}