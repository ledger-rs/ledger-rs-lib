/*!
 * Commodity definition
 *
 * commodity.cc
 */

use chrono::NaiveDateTime;

use crate::{amount::Amount, pool::CommodityIndex};
// use crate::pool::CommodityIndex;

#[derive(Debug, PartialEq)]
pub struct Commodity {
    pub symbol: String,
    /// Index in the commodity graph.
    pub graph_index: Option<CommodityIndex>,
    // precision
    pub name: Option<String>,
    pub note: Option<String>,
    // smaller: Option<Amount>
    // larger: Option<Amount>
    // value_expr: Option<>

    // commodity_pool
    // annotated_commodity
    // parent: *const CommodityPool,
    // qualified_symbol: Option<String>,
    pub annotated: bool,
}

impl Commodity {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            graph_index: None,
            name: None,
            note: None,
            annotated: false,
        }
    }
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
    pub price: Amount,
}

impl PricePoint {
    pub fn new(when: NaiveDateTime, price: Amount) -> Self {
        Self { when, price }
    }
}

#[cfg(test)]
mod tests {
    use super::Commodity;

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
