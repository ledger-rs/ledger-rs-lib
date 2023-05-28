use chrono::NaiveDateTime;

use crate::amount::Amount;

/**
 * commodity.cc
 */

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
    // annotated: bool
}

impl Commodity {
    pub fn new(symbol: &str) -> Self {
        Self { symbol: symbol.to_owned() }
    }

    pub fn parse(symbol: &str) -> Option<Self> {
        if symbol.is_empty() {
            return None;
        }

        Some(Commodity::new(symbol))
    }
}

struct PricePoint {
    when: NaiveDateTime,
    price: Amount
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

}