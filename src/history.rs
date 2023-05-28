/**
 * history.h + .cc
 * Commodity price history
 * 
 * Commodities are nodes (vertices).
 * 
 * All edges are weights computed as the absolute difference between
 * the reference time of a search and a known price point.  A
 * filtered_graph is used to select the recent price point to the
 * reference time before performing the search.
 */

use std::collections::HashMap;

use chrono::NaiveDateTime;
use petgraph::{Graph, stable_graph::NodeIndex};

use crate::{amount::Amount, commodity::Commodity, pool::CommodityIndex};

type PriceMap = HashMap<NaiveDateTime, Amount>;

pub(crate) struct CommodityHistory {
    graph: Graph<Commodity, PriceMap>
}

impl CommodityHistory {
    pub fn new() -> Self {
        Self { graph: Graph::new() }
    }

    /// Adds the commodity to the commodity graph.
    pub fn add_commodity(&mut self, commodity: Commodity) -> CommodityIndex {
        self.graph.add_node(commodity)
    }

    pub fn add_price(&mut self, source: CommodityIndex, date: NaiveDateTime, price: Amount) {
        assert!(Some(source) != price.commodity_index);

        // self.graph.from_index(i)
        
        //self.pri
        // Add the rate.
        // let Some(target) = price.commodity_index;
        // self.graph.add_edge(source, target, price.quantity);
        
        todo!()
    }

    pub fn get_commodity(&self, index: NodeIndex) -> &Commodity {
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
    use chrono::Local;
    use rust_decimal_macros::dec;

    use crate::{amount::Amount, commodity::Commodity, pool::CommodityIndex};

    use super::CommodityHistory;

    #[test]
    fn test_adding_commodity() {
        let mut hist = CommodityHistory::new();
        let c = Commodity::new("EUR");

        // Act
        let cdty_index = hist.add_commodity(c);

        // Assert
        assert_eq!(1, hist.graph.node_count());
        assert_eq!("EUR", hist.graph.node_weight(cdty_index).unwrap().symbol);
    }

    #[test]
    fn test_get_commodity() {
        let mut hist = CommodityHistory::new();
        let c = Commodity::new("EUR");
        let id = hist.add_commodity(c);

    }

    #[test]
    fn test_adding_price() {
        // Arrange
        let mut hist = CommodityHistory::new();
        let commodity_index = CommodityIndex::new(5);
        let local = Local::now();
        let today = local.naive_local();
        let price = Amount::new(dec!(25), Some(commodity_index));

        // Act
        hist.add_price(commodity_index, today, price);

        // Assert
        assert_eq!(1, hist.graph.node_count());
    }

    // #[test]
    // fn test_adding_rate() {
    //     let mut hist = CommodityHistory::new();
    //     //hist.graph.no
    //     let eur = hist.graph.add_node("eur");
    //     let aud = hist.graph.add_node("aud");

    //     let x = hist.graph.add_edge(eur, aud, weight);
    //     // x.index();
    // }
}