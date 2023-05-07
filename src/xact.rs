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
    pub fn finalize(&self) {
        let mut balance = Amount::null();

        for post in &self.posts {
            // must balance?

            // amount = post.cost ? post.amount
            // for now, just use the amount
            //todo balance += post.amount;
        }
    }
}
