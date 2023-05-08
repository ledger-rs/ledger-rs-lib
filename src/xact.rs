use std::cell::RefCell;

use chrono::NaiveDate;
use rust_decimal_macros::dec;

use crate::{amount::Amount, post::Post, journal::Journal};

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

fn list_of_actions() {
    // TODO: Link post.xact
    //let xact_index = context.xact.unwrap();
    // post.xact_index = Some(xact_index);

    // TODO: Add post to the Journal's Posts collection.
    // let post_index = context.journal.add_post(post);

    // TODO: add to xact.posts
    // let xact = context.journal.xacts.get_mut(xact_index).unwrap();
    // xact.posts.push(post_index);

    // TODO: add to account.posts
}

/// Finalize transaction.
/// 
/// `bool xact_base_t::finalize()`
/// 
/// TODO: add posts to the Journal, create links to Account and Xact.
/// 
pub fn finalize(xact: Xact, posts: Vec<Post>, journal: &mut Journal) {
    let mut balance = Amount::null();
    // let mut null_post: Option<&mut Post> = None;
    // The pointer to the post that has no amount.
    let mut null_post: Option<usize> = None;

    for post_index in &xact.posts {
        let post = &journal.posts[*post_index];

        // must balance?

        // amount = post.cost ? post.amount
        // for now, just use the amount
        if !post.amount.is_none() {
            balance.add(post.amount.as_ref().unwrap());
        } else if null_post.is_some() {
            todo!()
        } else {
            null_post = Some(*post_index);
        }
    }

    // If there is only one post, balance against the default account if one has
    // been set.

    if null_post.is_some() {
        // If one post has no value at all, its value will become the inverse of
        // the rest.  If multiple commodities are involved, multiple posts are
        // generated to balance them all.
        log::debug!("There was a null posting");

        if let Some(i) = null_post {
            let post = journal.posts.get_mut(i).expect("mutable post");
            // use inverse amount
            post.amount = Some(balance.inverse());
            null_post = None;
        };
    }

    // Add a pointer to each posting to their related accounts

    for post_index in &xact.posts {
        // add a pointer to account: 
        // TODO: account.posts.add_post(post);
        // Add post to account's list of post references.
        // post.borrow_mut().account.posts.borrow_mut().push(post.borrow());
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use rust_decimal_macros::dec;

    use crate::{account::Account, xact::finalize, parser};

    /// finalize
    #[test]
    fn test_finalize() {
        let src = r#";
2023-05-05 Payee
    Expenses  25
    Assets

"#;
        let source = Cursor::new(src);
        let mut journal = parser::parse(source);
        let xact = &journal.xacts[0];

        // finalize(xact, &mut journal);

        // let mut post_index = xact.posts[0];
        // let post1 = &journal.posts[post_index];
        // let amount = post1.amount.unwrap();
        // assert_eq!(dec!(-25), amount.quantity);
        // assert_eq!(None, amount.commodity);
        // assert_eq!(Account::new("Assets"), post1.account);
        todo!("complete")
    }
}
