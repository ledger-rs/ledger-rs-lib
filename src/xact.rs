use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

use crate::{
    balance::Balance,
    journal::{Journal, PostIndex, XactIndex},
    parser,
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

            balance.add(amt);
        } else if null_post.is_some() {
            todo!()
        } else {
            null_post = Some(*post_index);
        }
    }

    // If there is only one post, balance against the default account if one has
    // been set.
    if xact.posts.len() == 1 {
        todo!("handle")
    }

    if null_post.is_none() && xact.posts.len() == 2 {
        // When an xact involves two different commodities (regardless of how
        // many posts there are) determine the conversion ratio by dividing the
        // total value of one commodity by the total value of the other.  This
        // establishes the per-unit cost for this post for both commodities.
        todo!("complete")
    }

    // if (has_date())
    {
        for post_index in &xact.posts {
            let p = journal.get_post(*post_index);
            if p.cost.is_none() {
                continue;
            }

            let Some(amt) = &p.amount else {panic!("No amount found on the posting")};
            let Some(cost) = &p.cost else {panic!("No cost found on the posting")};
            if amt.commodity_index == cost.commodity_index {
                panic!("A posting's cost must be of a different commodity than its amount");
            }

            // Cost breakdown
            // TODO: virtual cost does not create a price
            let today = NaiveDateTime::new(Local::now().date_naive(), NaiveTime::MIN);
            let breakdown = journal
                .commodity_pool
                .exchange_breakdown(amt, cost, false, true, today, &journal);

            todo!("complete")
        }
    }

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
