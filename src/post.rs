/*!
 * Posting
 */

use crate::{
    amount::Amount,
    journal::{AccountIndex, XactIndex},
};

#[derive(Debug, PartialEq)]
pub struct Post {
    /// Pointer to the Account.
    // pub account: AccountIndex,
    pub account_index: AccountIndex,
    /// Pointer to the Xact.
    pub xact: XactIndex,

    pub amount: Option<Amount>,
    pub cost: Option<Amount>,
    // given_cost
    // assigned_amount
    // checkin
    // checkout

    pub note: Option<String>,
}

impl Post {
    /// Creates a Post from post tokens.
    pub fn new(
        account_index: AccountIndex,
        xact_index: XactIndex,
        amount: Option<Amount>,
        cost: Option<Amount>,
        note: Option<&str>,
    ) -> Self {
        Self {
            account_index,
            xact: xact_index,
            amount,
            cost,
            note: match note {
                Some(content) => Some(content.to_owned()),
                None => None,
            }
        }
    }

    pub fn add_note(&mut self, note: &str) {
        self.note = Some(note.into());
    }
}

#[cfg(test)]
mod tests {
}
