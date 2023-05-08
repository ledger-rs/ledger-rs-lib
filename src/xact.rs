use std::cell::RefCell;

use chrono::NaiveDate;
use rust_decimal_macros::dec;

use crate::{amount::Amount, journal::Journal, post::Post};

pub struct Xact {
    pub date: Option<NaiveDate>,
    pub payee: String,
    // pub posts: Vec<Post>,
    pub posts: Vec<usize>,
    pub note: Option<String>,
    // pub balance: Amount,
}

impl Xact {
    pub fn new(date: Option<NaiveDate>, payee: &str, note: Option<String>) -> Self {
        // code: Option<String>

        Self {
            payee: payee.to_owned(),
            note,
            posts: vec![],
            date,
            // balance: Amount::null(),
        }
    }

    // pub fn add_post(&mut self, post: Post) {
    //     self.posts.push(post);
    // }
}

/// Finalize transaction.
/// Adds the Xact and the Posts to the Journal.
///
/// `bool xact_base_t::finalize()`
///
/// TODO: add posts to the Journal, create links to Account and Xact.
///
pub fn finalize(xact: Xact, mut posts: Vec<Post>, journal: &mut Journal) {
    let mut balance = Amount::null();
    // The pointer to the post that has no amount.
    let mut null_post: Option<&mut Post> = None;

    // Balance
    for post in posts.iter_mut() {
        // must balance?

        // amount = post.cost ? post.amount
        // for now, just use the amount
        if !post.amount.is_none() {
            balance.add(post.amount.as_ref().unwrap());
        } else if null_post.is_some() {
            todo!()
        } else {
            null_post = Some(post);
        }
    }

    // If there is only one post, balance against the default account if one has
    // been set.

    // Handle null-amount post.
    if null_post.is_some() {
        // If one post has no value at all, its value will become the inverse of
        // the rest.  If multiple commodities are involved, multiple posts are
        // generated to balance them all.
        log::debug!("There was a null posting");

        let post = null_post.unwrap();
        // use inverse amount
        post.amount = Some(balance.inverse());
        null_post = None;
    }

    // Linking

    // Move the Xact into the Journal's Xacts collection.
    let xact_index = journal.add_xact(xact);

    // Link post.xact->xact
    for post in posts.iter_mut() {
        post.xact_index = Some(xact_index);
    }

    let mut post_indices = vec![];
    // Add posts to the Journal's Posts collection.
    for post in posts {
        let post_index = journal.add_post(post);
        post_indices.push(post_index);
    }

    // Add a pointer to each posting to their related accounts

    let xact = journal.xacts.get_mut(xact_index).unwrap();
    for post_index in post_indices {
        // add to xact.posts
        xact.posts.push(post_index);

        // add a pointer to account:
        // TODO: account.posts.add_post(post);
        // Add post to account's list of post references.
        // post.borrow_mut().account.posts.borrow_mut().push(post.borrow());
        // todo!("handle account")
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{amount::Amount, context::ParsingContext, post::Post, xact::finalize, account::Account};

    use super::Xact;

    fn setup() -> (Xact, Vec<Post>) {
        let xact = Xact::new(None, "payee", None);

        let post1 = Post::new("Expenses", Some(Amount::new(dec!(25), None)));
        let post2 = Post::new("Assets", None);
        let posts = vec![post1, post2];

        (xact, posts)
    }

    /// finalize
    #[test]
    fn test_finalize() {
        // Arrange
        let mut context = ParsingContext::new();
        let (xact, posts) = setup();

        // Act
        finalize(xact, posts, &mut context.journal);

        // Assert
        // let xact = context.journal.xacts.last().unwrap();
        let post1 = &context.journal.posts[0];
        assert_eq!(Account::new("Expenses"), post1.account);
        let amount = post1.amount.as_ref().unwrap();
        assert_eq!(dec!(25), amount.quantity);
        assert_eq!(None, amount.commodity);

        let post2 = &context.journal.posts[1];
        assert_eq!(Account::new("Assets"), post2.account);
        // assert_eq!(None, post2.amount);
        // The amount has been automatically recalculated to offset the first one.
        let amount = post2.amount.as_ref().unwrap();
        assert_eq!(dec!(-25), amount.quantity);
        assert_eq!(None, amount.commodity);
    }
}
