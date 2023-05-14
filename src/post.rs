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
    /// Creates a Post from post tokens.
    pub fn new(
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
