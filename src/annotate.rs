/*!
 * Types for annotating commodities
 *
 * annotate.h
 *
 * Annotations normally represent the Lot information: date, price.
 */

use anyhow::Error;
use chrono::NaiveDate;

use crate::{amount::{Amount, Quantity}, parser::ISO_DATE_FORMAT, journal::Journal};

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

    pub fn parse(date: &str, quantity: &str, commodity_symbol: &str, journal: &mut Journal) -> Result<Self, Error> {
        // parse amount
        let commodity = journal.commodity_pool.find_or_create(commodity_symbol, None);

        let price = if let Some(quantity) = Quantity::from_str(quantity) {
            Some(Amount::new(quantity, Some(commodity)))
        } else {
            None
        };
        
        let result = Self {
            price: price,
            date: match date.is_empty() {
                true => None,
                false => {
                    let d =
                        NaiveDate::parse_from_str(date, ISO_DATE_FORMAT).expect("successful parse");
                    Some(d)
                }
            },
        };
        
        Ok(result)
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

        let actual = Annotation::parse("2023-01-10", "20", expected_symbol, &mut journal).unwrap();

        assert_eq!("2023-01-10", actual.date.unwrap().to_string());
        assert_eq!(actual.price.unwrap().quantity, 20.into());

        let symbol = actual.price.unwrap().get_commodity().unwrap().symbol.to_owned();
        assert_eq!(expected_symbol, symbol);
    }
}
