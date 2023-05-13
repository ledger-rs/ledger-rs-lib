use chrono::NaiveDate;

use crate::{amount::Amount, journal::Journal, parser, post::Post};

pub struct Xact {
    pub date: Option<NaiveDate>,
    pub aux_date: Option<NaiveDate>,
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
            aux_date: None,
            // balance: Amount::null(),
        }
    }

    pub fn create(date: &str, aux_date: &str, payee: &str, note: &str) -> Self {
        let _date = if date.is_empty() {
            None
        } else {
            Some(parser::parse_date(date))
        };

        let _aux_date = if aux_date.is_empty() {
            None
        } else {
            Some(parser::parse_date(aux_date))
        };

        let _payee = if payee.is_empty() {
            "Unknown Payee".to_string()
        } else {
            payee.to_string()
        };

        let _note = if note.is_empty() {
            None
        } else {
            Some(note.to_string())
        };

        Self {
            date: _date,
            payee: _payee,
            posts: vec![],
            note: _note,
            aux_date: _aux_date,
        }
    }
}

/// Finalize transaction.
/// Adds the Xact and the Posts to the Journal.
///
/// `bool xact_base_t::finalize()`
///
/// TODO: add posts to the Journal, create links to Account and Xact.
///
pub fn finalize(xact: Xact, mut posts: Vec<Post>, journal: &mut Journal) {
    let mut balance: Option<Amount> = None;
    // The pointer to the post that has no amount.
    let mut null_post: Option<&mut Post> = None;

    // Balance
    for post in posts.iter_mut() {
        // must balance?

        // amount = post.cost ? post.amount
        // for now, just use the amount
        if !post.amount.as_ref().unwrap().is_null() {
            if balance.is_none() {
                let initial_amount = Amount::copy_from(&post.amount.as_ref().unwrap());
                balance = Some(initial_amount);
            } else {
                balance.as_mut().unwrap().add(&post.amount.as_ref().unwrap());
            }
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
        post.amount = Some(balance.unwrap().inverse());
        null_post = None;
    }

    // TODO: Process Commodities?
    // TODO: Process Account records from Posts.

    // Linking

    // Move the Xact into the Journal's Xacts collection.
    let xact_index = journal.add_xact(xact);

    // Link post.xact->xact
    for post in posts.iter_mut() {
        post.xact = xact_index;
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

    use crate::{
        account::Account, amount::Amount, context::ParsingContext, post::Post, xact::finalize,
    };

    use super::Xact;

    fn setup() -> (Xact, Vec<Post>) {
        let xact = Xact::new(None, "payee", None);

        let post1 = Post::new("Expenses");
        // Some(Amount::new(dec!(25), None, None)
        let post2 = Post::new("Assets");
        // None
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
        assert_eq!(Account::new("Expenses"), post1.account_temp);
        let amount = &post1.amount;
        assert_eq!(dec!(25), amount.as_ref().unwrap().quantity);
        assert_eq!(None, amount.as_ref().unwrap().commodity);

        let post2 = &context.journal.posts[1];
        assert_eq!(Account::new("Assets"), post2.account_temp);
        // assert_eq!(None, post2.amount);
        // The amount has been automatically recalculated to offset the first one.
        let amount = &post2.amount;
        assert_eq!(dec!(-25), amount.as_ref().unwrap().quantity);
        assert_eq!(None, amount.as_ref().unwrap().commodity);
    }
}
