/*!
 * Simple Model
 * Used for parsing only. This is when the client needs only to parse the Journal
 * file, without processing the directives.
 */

use chrono::NaiveDate;

use crate::amount::Quantity;

#[derive(Debug)]
pub struct Xact {
    pub date: Option<NaiveDate>,
    pub aux_date: Option<NaiveDate>,
    pub payee: String,
    pub posts: Vec<Posting>,
    pub note: Option<String>,
}

impl Xact {
    pub fn new() -> Self {
        Xact { date: None, aux_date: None, payee: "".into(), posts: vec![], note: None }
    }
}

#[derive(Debug)]
pub struct Posting {
    pub account: String,
    pub amount: Option<SimpleAmount>,
    pub cost: Option<SimpleAmount>,
    pub note: Option<String>,
}

#[derive(Debug)]
pub struct SimpleAmount {
    pub quantity: Quantity,
    pub commodity: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::amount::Quantity;

    use super::{Xact, Posting, SimpleAmount};

    #[test]
    fn test_create_simple_xact() {
        let mut xact = Xact {
            date: None,
            aux_date: None,
            payee: "Payee".into(),
            posts: vec![],
            note: None,
        };

        xact.posts.push(Posting {
            account: "Income".into(),
            amount: Some(SimpleAmount {
                quantity: Quantity::from_str("12.45").unwrap(),
                commodity: Some("AUD".into()),
            }),
            cost: None,
            note: None,
        });

        // assert
        assert!(xact.posts.len() == 1);
    }
}