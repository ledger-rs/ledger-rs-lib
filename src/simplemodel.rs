/*!
 * Simple Model
 * Used for parsing only. This is when the client needs only to parse the Journal
 * file, without processing the directives.
 */

use chrono::NaiveDate;

use crate::amount::Quantity;

pub struct SimpleXact {
    pub date: Option<NaiveDate>,
    pub aux_date: Option<NaiveDate>,
    pub payee: String,
    pub posts: Vec<SimplePost>,
    pub note: Option<String>,
}

pub struct SimplePost {
    pub account: String,
    pub amount: Option<SimpleAmount>,
    pub cost: Option<SimpleAmount>,
    pub note: Option<String>,
}

pub struct SimpleAmount {
    pub quantity: Quantity,
    pub commodity: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::amount::Quantity;

    use super::{SimpleXact, SimplePost, SimpleAmount};

    #[test]
    fn test_create_simple_xact() {
        let mut xact = SimpleXact {
            date: None,
            aux_date: None,
            payee: "Payee".into(),
            posts: vec![],
            note: None,
        };

        xact.posts.push(SimplePost {
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