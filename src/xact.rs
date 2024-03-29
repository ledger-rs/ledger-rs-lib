/*!
 * Transaction module
 *
 * Transaction, or Xact abbreviated, is the main element of the Journal.
 * It contains contains Postings.
 */

use chrono::NaiveDate;

use crate::{balance::Balance, journal::Journal, parser, post::Post};

#[derive(Debug)]
pub struct Xact {
    pub journal: *const Journal,
    pub date: Option<NaiveDate>,
    pub aux_date: Option<NaiveDate>,
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
            date,
            aux_date: None,
            journal: std::ptr::null(),
            posts: vec![],
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
            note: _note,
            aux_date: _aux_date,
            journal: std::ptr::null(),
            posts: vec![],
        }
    }

    pub fn add_note(&mut self, note: &str) {
        self.note = Some(note.into());
    }

    /// Adds the post to the collection and returns the reference to it.
    pub fn add_post(&mut self, mut post: Post) -> &Post {
        post.xact_ptr = self as *const Xact;
        self.posts.push(post);

        self.posts.last().unwrap()
    }
}

impl Default for Xact {
    fn default() -> Self {
        Self {
            journal: std::ptr::null(),
            date: Default::default(),
            aux_date: Default::default(),
            payee: Default::default(),
            posts: Default::default(),
            note: Default::default(),
        }
    }
}

/// Finalize transaction.
/// Adds the Xact and the Posts to the Journal.
///
/// `bool xact_base_t::finalize()`
///
pub fn finalize(xact_ptr: *const Xact, journal: &mut Journal) {
    // Scan through and compute the total balance for the xact.  This is used
    // for auto-calculating the value of xacts with no cost, and the per-unit
    // price of unpriced commodities.

    let mut balance = Balance::new();
    // The pointer to the post that has no amount.
    let mut null_post: *mut Post = std::ptr::null_mut();
    // let xact = journal.xacts.get_mut(xact_index).expect("xact");
    let xact: &mut Xact;
    unsafe {
        xact = &mut *(xact_ptr.cast_mut());
    }

    // Balance
    for post in &mut xact.posts {
        // must balance?
        // if (! post->must_balance())

        log::debug!("finalizing {:?}", post);

        let amount = if post.cost.is_some() {
            post.cost
        } else {
            post.amount
        };

        if amount.is_some() {
            // Add to balance.
            let Some(amt) = &amount
                else {panic!("should not happen")};

            balance.add(amt);
        } else if !null_post.is_null() {
            todo!()
        } else {
            null_post = post as *mut Post;
        }
    }

    // If there is only one post, balance against the default account if one has
    // been set.
    if xact.posts.len() == 1 {
        todo!("handle")
    }

    if null_post.is_null() && balance.amounts.len() == 2 {
        // When an xact involves two different commodities (regardless of how
        // many posts there are) determine the conversion ratio by dividing the
        // total value of one commodity by the total value of the other.  This
        // establishes the per-unit cost for this post for both commodities.

        let mut top_post: Option<&Post> = None;
        for post in &xact.posts {
            if post.amount.is_some() && top_post.is_none() {
                top_post = Some(post);
            }
        }

        // if !saw_cost && top_post
        if top_post.is_some() {
            // log::debug("there were no costs, and a valid top_post")

            // We need a separate readonly reference for reading the amounts.
            let const_bal: &Balance;
            unsafe {
                let const_ptr = &balance as *const Balance;
                const_bal = &*const_ptr;
            }

            let mut x = const_bal.amounts.iter().nth(0).unwrap();
            let mut y = const_bal.amounts.iter().nth(1).unwrap();

            // if x && y
            if !x.is_zero() && !y.is_zero() {
                if x.get_commodity() != top_post.unwrap().amount.unwrap().get_commodity() {
                    (x, y) = (y, x);
                }

                let comm = x.get_commodity();
                let per_unit_cost = (*y / *x).abs();

                for post in &mut xact.posts {
                    let amt = post.amount.unwrap();

                    if amt.get_commodity() == comm {
                        // todo!("check below");
                        balance -= amt;
                        post.cost = Some(per_unit_cost * amt);
                        balance += post.cost.unwrap();
                    }
                }
            }
        }
    }

    // if (has_date())
    {
        for p in &mut xact.posts {
            // let p = journal.posts.get_mut(*post_index).unwrap();
            if p.cost.is_none() {
                continue;
            }

            let Some(amt) = &p.amount else {panic!("No amount found on the posting")};
            let Some(cost) = &p.cost else {panic!("No cost found on the posting")};
            if amt.get_commodity() == cost.get_commodity() {
                panic!("A posting's cost must be of a different commodity than its amount");
            }

            {
                // Cost breakdown
                // todo: virtual cost does not create a price

                let moment = xact.date.unwrap().and_hms_opt(0, 0, 0).unwrap();
                let (breakdown, new_price_opt) =
                    journal.commodity_pool.exchange(amt, cost, false, moment);
                // add price(s)
                if let Some(new_price) = new_price_opt {
                    journal.commodity_pool.add_price_struct(new_price);
                }
                // TODO: this is probably redundant now?
                // if amt.commodity_index != cost.commodity_index {
                //     log::debug!("adding price amt: {:?} date: {:?}, cost: {:?}", amt.commodity_index, moment, cost);

                //     journal
                //         .commodity_pool
                //         .add_price(amt.commodity_index.unwrap(), moment, *cost);
                // }

                p.amount = Some(breakdown.amount);
            }
        }
    }

    // Handle null-amount post.
    if !null_post.is_null() {
        // If one post has no value at all, its value will become the inverse of
        // the rest.  If multiple commodities are involved, multiple posts are
        // generated to balance them all.

        log::debug!("There was a null posting");

        // let Some(null_post_index) = null_post
        //     else {panic!("should not happen")};
        // let Some(post) = journal.posts.get_mut(null_post_index)
        //     else {panic!("should not happen")};
        let post: &mut Post;
        unsafe {
            post = &mut *null_post;
        }

        // use inverse amount
        let amt = if balance.amounts.len() == 1 {
            // only one commodity
            let amt_bal = balance.amounts.iter().nth(0).unwrap();

            log::debug!("null-post amount reversing {:?}", amt_bal);

            amt_bal.inverse()
        } else {
            // TODO: handle option when there are multiple currencies and only one blank posting.

            todo!("check this option")
        };

        post.amount = Some(amt);

        null_post = std::ptr::null_mut();
    }

    // TODO: Process Commodities?
    // TODO: Process Account records from Posts.
}

#[cfg(test)]
mod tests {
    use crate::post::Post;

    use super::Xact;

    #[test]
    fn test_add_post() {
        let post = Post::new(std::ptr::null(), std::ptr::null(), None, None, None);
        let mut xact = Xact::default();

        // act
        xact.add_post(post);

        // assert
        assert_eq!(1, xact.posts.len());
        assert!(!xact.posts[0].xact_ptr.is_null());
    }
}
