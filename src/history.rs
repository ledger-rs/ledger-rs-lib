/**
 * history.h + .cc
 * Commodity price history
 */

use std::collections::HashMap;

use chrono::NaiveDateTime;
use petgraph::{Graph, stable_graph::NodeIndex};
use rust_decimal::Decimal;

use crate::{amount::Amount, commodity::Commodity, journal::CommodityIndex};

type PriceMap = HashMap<NaiveDateTime, Amount>;

// type CommodityHistory = Graph<String, Decimal>;

pub(crate) struct CommodityHistory {
    graph: Graph<CommodityIndex, Decimal>
}

impl CommodityHistory {
    pub fn new() -> Self {
        Self { graph: Graph::new() }
    }

    /// Adds the index of a commodity to the commodity graph.
    /// The actual commodity can be retrieved from the Pool.
    pub fn add_commodity(&mut self, index: CommodityIndex) -> NodeIndex {
        self.graph.add_node(index)
    }

    pub fn add_price(&self) {
        todo!()
    }

    pub fn map_prices(&self) {
        todo!()
    }

    pub fn find_price(&self) {
        todo!()
    }

    fn print_map(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::commodity::Commodity;
    use super::CommodityHistory;

    #[test]
    fn test_adding_commodity() {
        let mut hist = CommodityHistory::new();
        // let cdty = Commodity::new("EUR");

        // Act
        let cdty_index = hist.add_commodity(0);

        // Assert
        assert_eq!(1, hist.graph.node_count());
        assert_eq!(&0, hist.graph.node_weight(cdty_index).unwrap());
    }
}