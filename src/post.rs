use crate::{amount::Amount, account::Account, journal::{AccountIndex, XactIndex}};

/**
 * Posting
 */

 #[derive(Debug, PartialEq)]
pub struct Post {
    /// Pointer to the Account.
    pub account: AccountIndex,
    /// Pointer to the Xact.
    pub xact: XactIndex,

    // TODO: remove this temp field. Used just for testing.
    pub account_temp: Account,

    pub amount: Option<Amount>,
}

impl Post {
    pub fn new(account: &str) -> Self {
        Self {
            account: usize::MAX,
            xact: usize::MAX,

            account_temp: Account::new(account),
            amount: None,
        }
    }

    /// Creates a Post from post tokens.
    pub fn create_indexed(account_index: AccountIndex, xact_index: XactIndex, amount: Option<Amount>) -> Self {
        Self { account: account_index, xact: xact_index, amount, account_temp: Account::new("???") }
    }
}
