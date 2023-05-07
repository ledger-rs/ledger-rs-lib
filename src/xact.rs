use chrono::NaiveDate;
use rust_decimal_macros::dec;

use crate::{amount::Amount, post::Post};

pub struct Xact {
    pub date: Option<NaiveDate>,
    pub payee: String,
    pub posts: Vec<Post>,
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

    pub fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    /// Finalize transaction.
    /// bool xact_base_t::finalize()
    pub fn finalize(&mut self) {
        let mut balance = Amount::null();
        let mut null_post: Option<&mut Post> = None;
        // let mut null_post_index = -1;

        // for mut post in &self.posts {
        //     // must balance?

        //     // amount = post.cost ? post.amount
        //     // for now, just use the amount
        //     if !post.amount.is_none() {
        //         balance.add(post.amount.as_ref().unwrap());
        //     } else if null_post.is_some() {
        //         todo!()
        //     } else {
        //         null_post = Some(post);
        //     }
        // }

        // If there is only one post, balance against the default account if one has
        // been set.

        // if null_post.is_some() {
        //     // If one post has no value at all, its value will become the inverse of
        //     // the rest.  If multiple commodities are involved, multiple posts are
        //     // generated to balance them all.
        //     log::debug!("There was a null posting");

        //     if let Some(x) = null_post.as_mut() {
        //         x.amount = Some(Amount::null());
        //     };
        // }

        // if self.posts.into_iter().any(|p| p.amount.is_none()) {
        //     todo!()
        // }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use crate::{amount::Amount, post::Post};

    use super::Xact;

    /// finalize
    #[test]
    fn test_finalize() {
        let mut actual = Xact::new(None, "payee", None);
        actual.add_post(Post::new("Expenses", Some(Amount::new(dec!(25), None))));
        actual.add_post(Post::new("Assets", None));

        actual.finalize();

        assert!(false);
    }
}
