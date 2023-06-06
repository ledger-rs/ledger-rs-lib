/*!
 * Commodity price history
 *
 * history.h + .cc
 *
 * Commodities are nodes (vertices).
 *
 * All edges are weights computed as the absolute difference between
 * the reference time of a search and a known price point.  A
 * filtered_graph is used to select the recent price point to the
 * reference time before performing the search.
 */

use std::{collections::HashMap, ops::Add};

use chrono::NaiveDateTime;
use petgraph::{algo::dijkstra, stable_graph::NodeIndex, Graph, visit::EdgeIndexable};

use crate::{
    amount::{Amount, Decimal},
    commodity::Commodity,
    pool::CommodityIndex,
};

// type PriceMap = HashMap<NaiveDateTime, Amount>;
type PriceMap = HashMap<NaiveDateTime, Decimal>;

pub(crate) struct CommodityHistory {
    pub(crate) graph: Graph<Commodity, PriceMap>,
}

impl CommodityHistory {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
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
    pub fn add_price(
        &mut self,
        commodity_index: CommodityIndex,
        datetime: NaiveDateTime,
        price: Amount,
    ) {
        assert!(Some(commodity_index) != price.commodity_index);

        log::debug!(
            "adding price for {:?}, date: {:?}, price: {:?}",
            commodity_index,
            datetime,
            price
        );

        // edge = HashMap<NaiveDateTime, Decimal>
        let index = match self
            .graph
            .find_edge(commodity_index, price.commodity_index.unwrap())
        {
            Some(index) => index,
            None => {
                let dest = price.commodity_index.unwrap();
                self.graph.add_edge(commodity_index, dest, PriceMap::new())
            }
        };

        let prices = self.graph.edge_weight_mut(index).unwrap();

        // Add the price to the price history.
        prices.insert(datetime, price.quantity);
    }

    pub fn get_commodity(&self, index: NodeIndex) -> &Commodity {
        self.graph
            .node_weight(index)
            .expect("index should be valid")
    }

    pub fn get_commodity_mut(&mut self, index: NodeIndex) -> &mut Commodity {
        self.graph
            .node_weight_mut(index)
            .expect("index should be valid")
    }

    pub fn map_prices(&self) {
        todo!()
    }

    /// find_price(source, target, moment, oldest);
    pub fn find_price(
        &self,
        source: CommodityIndex,
        target: CommodityIndex,
        moment: NaiveDateTime,
    ) -> Price {
        assert_ne!(source, target);

        // let actual = dijkstra(&self.graph, source, Some(target), |e| *e.weight());

        todo!()
    }

    /// Finds the price
    /// i.e. 1 EUR = 1.10 USD
    /// source: EUR
    /// target: USD
    pub fn get_direct_price(&self, source: CommodityIndex, target: CommodityIndex) -> Option<&Decimal> {
        let direct = self.graph.find_edge(source, target);
        if let Some(edge_index) = direct {
            let price_history = self.graph.edge_weight(edge_index).unwrap();
            get_latest_price(price_history)
        } else {
            None
        }
    }

    fn print_map(&self) {
        todo!()
    }
}

/// Returns the latest (newest) price from the prices hashmap.
fn get_latest_price(prices: &HashMap<NaiveDateTime, Decimal>) -> Option<&Decimal> {
    if prices.is_empty() {
        return None;
    }

    let mut dates: Vec<&NaiveDateTime> = prices.keys().collect();
    dates.sort();

    let last_date = *dates.last().unwrap();

    prices.get(last_date)
}

/// Represents a price of a commodity.
/// i.e. (1) EUR = 1.20 AUD
///
/// Also used for price_point_t, which does not have the commodity_index.
///
#[derive(Debug)]
pub struct Price {
    /// The commodity being priced.
    pub commodity_index: CommodityIndex,
    /// Point in time at which the price is valid.
    pub datetime: NaiveDateTime,
    /// Price of the commodity. i.e. 1.20 AUD
    pub price: Amount,
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use petgraph::stable_graph::NodeIndex;

    use super::CommodityHistory;
    use crate::{
        amount::{Amount, Decimal},
        commodity::Commodity,
        journal::Journal,
        parser::{parse_amount, parse_datetime},
    };

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
        let price = Amount::new(25.into(), Some(usd));

        // Act
        hist.add_price(eur, today, price);

        // Assert
        assert_eq!(2, hist.graph.node_count());
        assert_eq!(1, hist.graph.edge_count());

        let edge = hist.graph.edge_weights().nth(0).unwrap();
        assert_eq!(&Decimal::from(25), edge.values().nth(0).unwrap());
    }

    #[test]
    fn test_index() {
        let mut graph = CommodityHistory::new();
        let x = graph.add_commodity(Commodity::new("EUR"));
        let y = x.index();
        let z = NodeIndex::new(y);

        assert_eq!(z, x);
    }

    #[test]
    fn test_find_price() {
        let journal = &mut Journal::new();
        let eur_index = journal.commodity_pool.create("EUR", None);
        let usd_index = journal.commodity_pool.create("USD", None);
        // add price
        let date = parse_datetime("2023-05-01").unwrap();
        let price = parse_amount("1.20 USD", journal).unwrap();
        journal
            .commodity_pool
            .commodity_history
            .add_price(eur_index, date, price);

        // act
        let actual = journal.commodity_pool.commodity_history.get_direct_price(eur_index, usd_index).unwrap();

        // assert
        // assert_eq!(eur_index, actual.commodity_index);
        // assert_eq!("2023-05-01 00:00:00", actual.datetime.to_string());
        // assert_eq!(actual.price.quantity, 1.20.into());
        assert_eq!(Decimal::from("1.20"), *actual);
    }
}
