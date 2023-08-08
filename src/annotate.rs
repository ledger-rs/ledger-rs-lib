/*!
 * Types for annotating commodities
 *
 * annotate.h
 *
 * Annotations normally represent the Lot information: date, price.
 */

use chrono::NaiveDate;

use crate::{amount::Amount, parser::ISO_DATE_FORMAT, journal::Journal, commodity::Commodity};

pub struct Annotation {
    /// Price per unit. The {} value in the Lot syntax.
    pub price: Option<Amount>,
    /// The [] date in the Lot syntax.
    pub date: Option<NaiveDate>,
    // pub tag: Option<String>,
    // pub value_expr:
}

impl Annotation {
    pub fn new(price: Option<Amount>, date: Option<NaiveDate>) -> Self {
        // todo: add support for tags
        Self {
            price,
            date,
            // tag: None,
        }
    }

    pub fn parse(date: &str, quantity: &str, commodity_symbol: &str, journal: &mut Journal) -> Self {
        todo!("adjust the find method");

        // parse amount
        // let commodity_index = journal.commodity_pool.find_or_create(commodity_symbol, None);
        let commodity = &Commodity::new("Test");

        let amount = Amount::parse(quantity, Some(commodity));
        Self {
            price: amount,
            date: match date.is_empty() {
                true => None,
                false => {
                    let d =
                        NaiveDate::parse_from_str(date, ISO_DATE_FORMAT).expect("successful parse");
                    Some(d)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::journal::Journal;

    use super::Annotation;

    #[test]
    fn test_parsing() {
        let mut journal = Journal::new();
        let expected_symbol = "EUR";

        let actual = Annotation::parse("2023-01-10", "20", expected_symbol, &mut journal);

        assert_eq!("2023-01-10", actual.date.unwrap().to_string());
        assert_eq!(actual.price.unwrap().quantity, 20.into());

        let symbol = actual.price.unwrap().get_commodity().unwrap().symbol.to_owned();
        assert_eq!(expected_symbol, symbol);
    }
}
