/*!
 * Transaction module
 *
 * Transaction, or Xact abbreviated, is the main element of the Journal.
 * It contains contains Postings.
 */

use std::{cell::RefCell, rc::Rc};

use chrono::NaiveDate;

use crate::{
    balance::Balance,
    journal::{Journal, XactIndex},
    parser,
    post::Post,
};

pub struct Xact {
    pub date: Option<NaiveDate>,
    pub aux_date: Option<NaiveDate>,
    pub payee: String,
    pub posts: Vec<Rc<RefCell<Post>>>,
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

    pub fn add_note(&mut self, note: &str) {
        self.note = Some(note.into());
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
    let mut null_post: Option<Rc<RefCell<Post>>> = None;
    let xact = journal.xacts.get(xact_index).expect("xact");

    // Balance
    for post_index in &xact.posts {
        // must balance?

        // let post = journal.posts.get(*post_index).expect("post");
        let post = post_index.borrow();

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
        } else if null_post.is_some() {
            todo!()
        } else {
            // null_post = Some(*post_index);
            null_post = Some(post_index.clone());
        }
    }

    // If there is only one post, balance against the default account if one has
    // been set.
    if xact.posts.len() == 1 {
        todo!("handle")
    }

    if null_post.is_none() && balance.amounts.len() == 2 {
        // When an xact involves two different commodities (regardless of how
        // many posts there are) determine the conversion ratio by dividing the
        // total value of one commodity by the total value of the other.  This
        // establishes the per-unit cost for this post for both commodities.

        let mut top_post: Option<Rc<RefCell<Post>>> = None;
        for i in &xact.posts {
            // let post = journal.get_post(*i);
            let post = i.clone();
            if post.borrow().amount.is_some() && top_post.is_none() {
                top_post = Some(post);
            }
        }

        // if !saw_cost && top_post
        if top_post.is_some() {
            // log::debug("there were no costs, and a valid top_post")

            let mut x = balance.amounts.iter().nth(0).unwrap();
            let mut y = balance.amounts.iter().nth(1).unwrap();

            // if x && y
            if !x.is_zero() && !y.is_zero() {
                let Some(p) = top_post.clone() else {panic!("oops")};
                if x.commodity_index != p.borrow().amount.unwrap().commodity_index {
                    (x, y) = (y, x);
                }

                let comm = x.commodity_index;
                let per_unit_cost = (*y / *x).abs();

                for i in &xact.posts {
                    // let post = journal.posts.get_mut(*i).unwrap();
                    let post = i;
                    let amt = post.borrow().amount.unwrap();

                    if amt.commodity_index == comm {
                        balance -= amt;
                        post.borrow_mut().cost = Some(per_unit_cost * amt);
                        balance += post.borrow().cost.unwrap();
                    }
                }
            }
        }
    }

    // if (has_date())
    {
        for post_index in &xact.posts {
            // let p = journal.posts.get_mut(*post_index).unwrap();
            let p = post_index.clone();
            if p.borrow().cost.is_none() {
                continue;
            }

            let Some(amt) = p.borrow().amount else {panic!("No amount found on the posting")};
            let Some(cost) = p.borrow().cost else {panic!("No cost found on the posting")};
            if amt.commodity_index == cost.commodity_index {
                panic!("A posting's cost must be of a different commodity than its amount");
            }

            {
                // Cost breakdown
                // todo: virtual cost does not create a price

                let breakdown = {
                    let moment = xact.date.unwrap().and_hms_opt(0, 0, 0).unwrap();
                    let (breakdown, new_price_opt) =
                        journal.commodity_pool.exchange(&amt, &cost, false, moment);
                    // add price(s)
                    if let Some(new_price) = new_price_opt {
                        journal.commodity_pool.add_price_struct(new_price);
                    }

                    breakdown
                };
                // TODO: this is probably redundant now?
                // if amt.commodity_index != cost.commodity_index {
                //     log::debug!("adding price amt: {:?} date: {:?}, cost: {:?}", amt.commodity_index, moment, cost);

                //     journal
                //         .commodity_pool
                //         .add_price(amt.commodity_index.unwrap(), moment, *cost);
                // }

                p.borrow_mut().amount = Some(breakdown.amount);
            }
        }
    }

    // Handle null-amount post.
    if null_post.is_some() {
        // If one post has no value at all, its value will become the inverse of
        // the rest.  If multiple commodities are involved, multiple posts are
        // generated to balance them all.

        log::debug!("There was a null posting");

        // let Some(null_post_index) = null_post
        //     else {panic!("should not happen")};
        // let Some(post) = journal.posts.get_mut(null_post_index)
        //     else {panic!("should not happen")};
        let Some(post) = null_post
            else {panic!("should not happen")};

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

        post.borrow_mut().amount = Some(amt);
        null_post = None;
    }

    // TODO: Process Commodities?
    // TODO: Process Account records from Posts.
}
