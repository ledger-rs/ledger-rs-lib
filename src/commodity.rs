/*!
 * Commodity definition
 * 
 * commodity.cc
 */

use chrono::NaiveDateTime;

use crate::amount::Amount;

#[derive(Debug, PartialEq)]
pub struct Commodity {
    pub symbol: String,
    // graph_index: Option
    // precision
    // name: Option<String>
    // note: Option<String>
    // smaller: Option
    // larger: Option
    // value_expr: Option

    // commodity_pool
    // annotated_commodity
    // parent
    // qualified_symbol: Option<String>
    pub annotated: bool
}

impl Commodity {
    pub fn new(symbol: &str) -> Self {
        Self { symbol: symbol.to_owned(), annotated: false }
    }

    // pub fn parse(symbol: &str) -> Option<Self> {
    //     if symbol.is_empty() {
    //         return None;
    //     }
    //     Some(Commodity::new(symbol))
    // }
}

/// commodity.cc
/// 
pub(crate) fn find_price(commodity: &Commodity, moment: NaiveDateTime, oldest: NaiveDateTime) {
    // if commodity
    let target = commodity;

    // memoized_price_entry entry(moment, oldest, commodity)

    // memoized_price_map map<memoized_price_entry, optional<price_point_t> >
    // commodity.price_map.find(entry)

    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PricePoint {
    pub when: NaiveDateTime,
    pub price: Amount
}

impl PricePoint {
    pub fn new(when: NaiveDateTime, price: Amount) -> Self {
        Self { when, price }
    }
}

#[cfg(test)]
mod tests {
    use super::{Commodity, find_price};

    #[test]
    fn test_comparison() {
        let c1 = Commodity::new("EUR");
        let c2 = Commodity::new("EUR");

        assert!(c1 == c2);
    }

    #[test]
    fn test_comparison_ne() {
        let c1 = Commodity::new("EUR");
        let c2 = Commodity::new("GBP");

        assert!(c1 != c2);
    }

    #[test]
    fn test_find_price() {
        // arrange

        // act
        // let actual = find_price(commodity, moment, oldest);

        // assert
        todo!()
    }
}