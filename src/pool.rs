/**
 * Commodity Pool
 */
use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::{commodity::Commodity, journal::CommodityIndex, amount::Amount};

pub(crate) struct CommodityPool {
    /// Map (symbol, commodity)
    commodities: HashMap<String, Commodity>,
    // annotated_commodities: HashMap<String, Commodity>,

    // commodity_price_history: CommodityHistory
    // null_commodity: Commodity
    // default_commodity: Commodity

    // pricedb
}

impl CommodityPool {
    pub fn new() -> Self {
        Self {
            commodities: HashMap::new(),
        }
    }

    pub fn create(&mut self, symbol: &str) {
        // todo: handle double quotes

        let c = Commodity::new(symbol);

        self.commodities.insert(symbol.to_owned(), c);

        // TODO: add price history
        // commodity_price_history.add_commodity(*commodity.get());
    }

    pub fn find(&self, symbol: &str) -> Option<&Commodity> {
        self.commodities.get(symbol)
    }

    // pub fn find_or_create(&mut self, symbol: &str) -> &Commodity {
    //     match self.commodities.get(symbol) {
    //         Some(cdty) => return cdty,
    //         None => {
    //             self.create(symbol);
    //             return self.commodities.get(symbol).unwrap();
    //         }
    //     }
    // }

    pub fn exchange(&self, commodity: &Commodity, per_unit_cost: Amount, moment: NaiveDateTime) {
        todo!()
    }

    pub fn parse_price_directive() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::CommodityPool;

    #[test]
    fn test_adding_commodity() {
        let symbol = "EUR";
        let mut pool = CommodityPool::new();

        // Act
        pool.create(symbol);

        // Assert
        assert_eq!(1, pool.commodities.len());
        assert!(pool.commodities.contains_key("EUR"));
        // assert_eq!(Some(symbol), pool.commodities.get(symbol));
    }
}
