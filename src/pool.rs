/**
 * Commodity Pool
 */
use std::collections::HashMap;

use chrono::NaiveDateTime;
use petgraph::stable_graph::NodeIndex;

use crate::{amount::Amount, commodity::Commodity, history::CommodityHistory};

/// Commodity Index is the index of the node in the history graph.
pub type CommodityIndex = NodeIndex;

pub struct CommodityPool {
    /// Map (symbol, commodity)
    pub(crate) commodities: HashMap<String, NodeIndex>,
    // annotated_commodities: HashMap<String, Commodity>,
    pub(crate) commodity_history: CommodityHistory,
    // null_commodity: Commodity
    // default_commodity: Commodity

    // pricedb
}

impl CommodityPool {
    pub fn new() -> Self {
        Self {
            commodities: HashMap::new(),
            commodity_history: CommodityHistory::new(),
        }
    }

    /// Creates a new Commodity for the given Symbol.
    pub fn create(&mut self, symbol: &str) -> CommodityIndex {
        // todo: handle double quotes

        let c = Commodity::new(symbol);

        // add to price history graph.
        let i = self.commodity_history.add_commodity(c);

        // Add index to map.
        self.commodities.insert(symbol.to_owned(), i);

        i
    }

    pub fn find_index(&self, symbol: &str) -> Option<&CommodityIndex> {
        self.commodities.get(symbol)
    }

    pub fn find_commodity(&self, symbol: &str) -> Option<&Commodity> {
        match self.commodities.get(symbol) {
            Some(i) => Some(self.commodity_history.get_commodity(*i)),
            None => None,
        }
    }

    pub fn find_or_create(&mut self, symbol: &str) -> Option<CommodityIndex> {
        if symbol.is_empty() {
            return None;
        }

        match self.find_index(symbol) {
            Some(i) => Some(*i),
            None => Some(self.create(symbol)),
        }
    }

    pub fn get_commodity(&self, index: CommodityIndex) -> &Commodity {
        self.commodity_history.get_commodity(index)
    }

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

#[cfg(test)]
mod tests_algos {
    use petgraph::{
        algo::{bellman_ford, dijkstra, floyd_warshall},
        dot::Dot,
        Graph,
    };

    #[test]
    fn test_pet_graph() {
        // Arrange
        let mut hist = Graph::<&str, &str>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(aud, eur, "1.65");
        hist.add_edge(aud, usd, "1.30");

        // Act
        // Given the adge eur->aud->usd, get the rate eur/usd
        let dot = format!("{:?}", Dot::new(&hist));

        assert!(!dot.is_empty());
    }

    /// Test the Dijkstra algorithm, the shortest path between the nodes / commodities.
    #[test]
    fn test_dijkstra() {
        // Arrange
        let mut hist = Graph::<&str, f32>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(eur, aud, 0.85);
        hist.add_edge(aud, usd, 1.30);

        // Act
        let actual = dijkstra(&hist, eur, Some(usd), |_| 1);

        // Assert
        assert!(!actual.is_empty());
        // eur->aud->usd has three nodes.
        assert_eq!(3, actual.len());
    }

    /// Dijkstra algorithm should be enough for our purpose. It just needs to give us the shortest
    /// path between the desired currencies. The rates are all positive.
    /// However, this test is wrong, since it just adds edges, which is not what we need.
    #[test]
    fn test_exchange_with_dijkstra() {
        // Arrange
        let mut hist = Graph::<&str, f32>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(eur, aud, 1.65);
        hist.add_edge(aud, usd, 0.6520);

        // Act
        let actual = dijkstra(&hist, eur, Some(usd), |e| *e.weight());

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(3, actual.len());

        // The order is not guaranteed.
        // let (i, member_i32) = actual.iter().nth(0).unwrap();
        // let member = hist.node_weight(*i).unwrap();
        // assert_eq!("eur", *member);

        // let (i, member_i32) = actual.iter().nth(1).unwrap();
        // let member = hist.node_weight(*i).unwrap();
        // assert_eq!("aud", *member);

        // let (i, member_i32) = actual.iter().nth(2).unwrap();
        // let member = hist.node_weight(*i).unwrap();
        // assert_eq!("usd", *member);
    }

    /// Bellman-Ford algorhythm finds the shortest route but allows for negative edge cost.
    #[test]
    fn test_bellman_ford() {
        // Arrange
        let mut hist = Graph::<&str, f32>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(eur, aud, 0.85);
        hist.add_edge(aud, usd, 1.30);

        // Act
        let actual = bellman_ford(&hist, eur).unwrap();

        // Assert
        assert!(!actual.distances.is_empty());
        assert_eq!(3, actual.distances.len());
    }

    /// floyd_warshall algorithm
    /// Compute shortest paths in a weighted graph with positive or negative edge weights (but with no negative cycles)
    #[test]
    fn test_floyd_warshall() {
        // Arrange
        let mut hist = Graph::<&str, f32>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(eur, aud, 0.85);
        hist.add_edge(aud, usd, 1.30);

        // Act
        let actual = floyd_warshall(&hist, |_| 1).unwrap();

        assert!(!actual.is_empty());
    }

    // search for edge (direct exchange rate).
    #[test]
    fn test_search() {
        // Arrange
        let mut hist = Graph::<&str, f32>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(eur, aud, 1.65);
        hist.add_edge(aud, usd, 0.6520);

        // Act
        let actual = hist.find_edge(eur, aud);
        assert!(actual.is_some());

        let Some(euraud) = actual else {panic!()};
        let weight = hist.edge_weight(euraud).unwrap();
        assert_eq!(&1.65, weight);
    }
}
