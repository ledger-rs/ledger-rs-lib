use chrono::NaiveDate;

use crate::{
    amount::Amount,
    balance::Balance,
    journal::{Journal, PostIndex, XactIndex},
    parser,
    post::Post,
};

pub struct Xact {
    pub date: Option<NaiveDate>,
    pub aux_date: Option<NaiveDate>,
    pub payee: String,
    // pub posts: Vec<Post>,
    pub posts: Vec<PostIndex>,
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

    /// Creates a new Transaction from the scanned tokens.
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
pub fn finalize(xact_index: XactIndex, journal: &mut Journal) {
    // let mut balance: Option<Amount> = None;
    let mut balance = Balance::new();
    // The pointer to the post that has no amount.
    let mut null_post: Option<PostIndex> = None;
    let xact = journal.xacts.get(xact_index).expect("xact");

    // Balance
    for post_index in xact.posts.iter() {
        // must balance?

        let post = journal.posts.get(*post_index).expect("post");

        // amount = post.cost ? post.amount
        // for now, just use the amount
        if post.amount.is_some() {
            // Add to balance.
            let Some(amt) = &post.amount
                else {panic!("should not happen")};

            balance.add(amt.to_owned());
        } else if null_post.is_some() {
            todo!()
        } else {
            null_post = Some(*post_index);
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

        let Some(null_post_index) = null_post
            else {panic!("should not happen")};
        let Some(post) = journal.posts.get_mut(null_post_index)
            else {panic!("should not happen")};

        // use inverse amount
        let amt = if balance.amounts.len() == 1 {
            // only one commodity
            let amt_bal = balance.amounts.iter().nth(0).unwrap();

            amt_bal.inverse()
        } else {
            // TODO: handle option when there are multiple currencies and only one blank posting.
            todo!("check this option")
        };

        post.amount = Some(amt);
        null_post = None;
    }

    // TODO: Process Commodities?
    // TODO: Process Account records from Posts.
}
