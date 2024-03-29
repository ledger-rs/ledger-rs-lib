/*!
 * Posting
 */

use crate::{
    account::Account,
    amount::Amount,
    xact::Xact,
};

#[derive(Debug, PartialEq)]
pub struct Post {
    /// Pointer to the Account.
    pub account: *const Account,
    // pub account_index: AccountIndex,
    /// Pointer to the Xact.
    pub xact_ptr: *const Xact,
    // pub xact_index: XactIndex,

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
        account: *const Account,
        xact_ptr: *const Xact,
        amount: Option<Amount>,
        cost: Option<Amount>,
        note: Option<&str>,
    ) -> Self {
        Self {
            account,
            xact_ptr,
            amount,
            cost,
            note: match note {
                Some(content) => Some(content.to_owned()),
                None => None,
            },
        }
    }

    pub fn add_note(&mut self, note: &str) {
        self.note = Some(note.into());
    }
}

impl Default for Post {
    fn default() -> Self {
        Self {
            account: std::ptr::null(),
            xact_ptr: std::ptr::null(),
            amount: Default::default(),
            cost: Default::default(),
            note: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::account::Account;

    use super::Post;

    #[test]
    fn test_pointers() {
        const ACCT_NAME: &str = "Some Account";
        let mut post = Post::default();

        assert_eq!(std::ptr::null(), post.account);

        // Assign account.
        let acct = Account::new(ACCT_NAME);
        post.account = &acct as *const Account;

        unsafe {
            // println!("account is {:?}", *post.account);
            assert_eq!(ACCT_NAME, (*post.account).name);
        }
    }
}
