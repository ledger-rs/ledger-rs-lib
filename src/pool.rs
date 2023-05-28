/**
 * Commodity Pool
 */
use std::collections::HashMap;

use chrono::NaiveDateTime;
use petgraph::Graph;
use rust_decimal::Decimal;

use crate::{amount::Amount, commodity::Commodity};

pub(crate) struct CommodityPool {
    /// Map (symbol, commodity)
    commodities: HashMap<String, Commodity>,
    // annotated_commodities: HashMap<String, Commodity>,

    // commodity_price_history: CommodityHistory
    // commodity_history: Graph<String, Decimal>,
    commodity_history: CommodityHistory,
    // null_commodity: Commodity
    // default_commodity: Commodity

    // pricedb
}

impl CommodityPool {
    pub fn new() -> Self {
        Self {
            commodities: HashMap::new(),
            commodity_history: Graph::new(),
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

type CommodityHistory = Graph<String, Decimal>;

// impl CommodityHistory {
//     pub fn add_commodity(&self) {
//         todo!()
//     }
// }

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
mod exchange_tests {
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
        hist.add_edge(aud, eur, 1.65);
        hist.add_edge(aud, usd, 1.30);

        // Act
        let actual = dijkstra(&hist, eur, Some(usd), |_| 1);

        // Assert
        assert!(!actual.is_empty());
        // eur->aud->usd has one hop, aud.
        assert_eq!(1, actual.len());

        // let (member_i, member_i32) = actual.iter().next().unwrap();
        // let member = hist[member_i];
        // assert_eq!(&1, member_i32);
        // log::debug!("{:?}", member_i);
    }

    /// Dijkstra algorithm should be enough for our purpose. It just needs to give us the shortest
    /// path between the desired currencies. The rates are all positive.
    #[test]
    fn test_exchange_with_dijkstra() {
        // Arrange
        let mut hist = Graph::<&str, f32>::new();
        // edges are commodities
        let eur = hist.add_node("eur");
        let usd = hist.add_node("usd");
        let aud = hist.add_node("aud");
        // edges are prices / exchange rates
        hist.add_edge(aud, eur, 0.6074);
        hist.add_edge(aud, usd, 0.6520);

        // Act
        let actual = dijkstra(&hist, eur, Some(usd), |e| *e.weight());

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(1, actual.len());
        
        let (i, int) = actual.iter().next().unwrap();
        
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
}
