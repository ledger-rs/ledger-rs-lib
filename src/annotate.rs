/*!
 * Types for annotating commodities
 *
 * annotate.h
 *
 * Annotations normally represent the Lot information: date, price.
 */

use chrono::NaiveDate;

use crate::{amount::Amount, parser::ISO_DATE_FORMAT, journal::Journal};

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
        // parse amount
        let commodity_index = journal.commodity_pool.find_or_create(commodity_symbol, None);
        let amount = Amount::parse(quantity, commodity_index);
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
    use crate::{journal::Journal, pool::CommodityIndex};

    use super::Annotation;

    #[test]
    fn test_parsing() {
        let mut journal = Journal::new();

        let actual = Annotation::parse("2023-01-10", "20", "EUR", &mut journal);

        assert_eq!("2023-01-10", actual.date.unwrap().to_string());
        assert_eq!(actual.price.unwrap().quantity, 20.into());
        let commodity_index: CommodityIndex = 0.into();
        assert_eq!(actual.price.unwrap().commodity_index, Some(commodity_index));

        assert_eq!("EUR", journal.get_commodity(commodity_index).symbol);
    }
}
