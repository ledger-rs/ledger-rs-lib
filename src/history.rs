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
use rust_decimal::Decimal;

use crate::{amount::Amount, commodity::Commodity, pool::CommodityIndex};

// type PriceMap = HashMap<NaiveDateTime, Amount>;
type PriceMap = HashMap<NaiveDateTime, Decimal>;

pub(crate) struct CommodityHistory {
    pub(crate) graph: Graph<Commodity, PriceMap>
}

impl CommodityHistory {
    pub fn new() -> Self {
        Self { graph: Graph::new() }
    }

    /// Adds the commodity to the commodity graph.
    pub fn add_commodity(&mut self, commodity: Commodity) -> CommodityIndex {
        self.graph.add_node(commodity)
    }

    /// Adds a new price point.
    /// i.e. 1 EUR = 1.12 USD
    /// source: EUR
    /// date
    /// price: 1.12 USD
    pub fn add_price(&mut self, commodity: CommodityIndex, date: NaiveDateTime, price: Amount) {
        assert!(Some(commodity) != price.commodity_index);

        let index = match self.graph.find_edge(commodity, price.commodity_index.unwrap()) {
            Some(index) => index,
            None => {
                let dest = price.commodity_index.unwrap();
                self.graph.add_edge(commodity, dest, PriceMap::new())
            },
        };
        let prices = self.graph.edge_weight_mut(index).unwrap();

        // Add the rate.
        prices.insert(date, price.quantity);
    }

    pub fn get_commodity(&self, index: NodeIndex) -> &Commodity {
        self.graph.node_weight(index).expect("index should be valid")
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
    use petgraph::stable_graph::NodeIndex;
    use rust_decimal_macros::dec;

    use crate::{amount::Amount, commodity::Commodity};
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

        let actual = hist.get_commodity(id);

        assert_eq!("EUR", actual.symbol);
    }

    #[test]
    fn test_adding_price() {
        // Arrange
        let mut hist = CommodityHistory::new();
        let eur = hist.add_commodity(Commodity::new("EUR"));
        let usd = hist.add_commodity(Commodity::new("USD"));
        let local = Local::now();
        let today = local.naive_local();
        let price = Amount::new(dec!(25), Some(usd));

        // Act
        hist.add_price(eur, today, price);

        // Assert
        assert_eq!(2, hist.graph.node_count());
        assert_eq!(1, hist.graph.edge_count());

        let edge = hist.graph.edge_weights().nth(0).unwrap();
        assert_eq!(&dec!(25), edge.values().nth(0).unwrap());
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

    #[test]
    fn test_index() {
        let mut graph= CommodityHistory::new();
        let x = graph.add_commodity(Commodity::new("EUR"));
        let y = x.index();
        let z = NodeIndex::new(y);

        assert_eq!(z, x);
    }
}