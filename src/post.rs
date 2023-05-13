use crate::{
    account::Account,
    amount::Amount,
    journal::{AccountIndex, XactIndex},
};

/**
 * Posting
 */

#[derive(Debug, PartialEq)]
pub struct Post {
    /// Pointer to the Account.
    // pub account: AccountIndex,
    pub account_index: AccountIndex,
    /// Pointer to the Xact.
    pub xact: XactIndex,

    // TODO: remove this temp field. Used just for testing.
    // pub account_temp: Account,

    pub amount: Option<Amount>,
}

impl Post {
    pub fn new(account: &str) -> Self {
        Self {
            account_index: usize::MAX,
            xact: usize::MAX,

            amount: None,
        }
    }

    /// Creates a Post from post tokens.
    pub fn create_indexed(
        account_index: AccountIndex,
        xact_index: XactIndex,
        amount: Option<Amount>,
    ) -> Self {
        Self {
            account_index,
            xact: xact_index,
            amount,
        }
    }
}
