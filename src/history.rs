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

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use chrono::{Local, NaiveDateTime};
use petgraph::{algo::astar, stable_graph::NodeIndex, Graph};

use crate::{
    amount::{Amount, Quantity},
    commodity::{self, Commodity, PricePoint},
    pool::CommodityIndex,
};

type PriceMap = BTreeMap<NaiveDateTime, Quantity>;

/// commodity_history_t or commodity_history_impl_t?
// pub(crate) struct CommodityHistory {
//     pub(crate) graph: Graph<*const Commodity, PriceMap>,
// }

// pub(crate) type CommodityHistory = Graph<Commodity, PriceMap>;
pub(crate) struct CommodityHistory(Graph<*const Commodity, PriceMap>);

impl CommodityHistory {
    pub fn new() -> Self {
        Self(Graph::new())
    }

    /// Adds the commodity to the commodity graph.
    pub fn add_commodity(&mut self, commodity: *const Commodity) -> CommodityIndex {
        self.add_node(commodity)
    }

    /// Adds a new price point.
    /// i.e. 1 EUR = 1.12 USD
    /// source: EUR
    /// date
    /// price: 1.12 USD
    pub fn add_price(
        &mut self,
        source_ptr: *const Commodity,
        datetime: NaiveDateTime,
        price: Amount,
    ) {
        let source = commodity::from_ptr(source_ptr);
        assert!(Some(source) != price.get_commodity());

        log::debug!(
            "adding price for {:?}, date: {:?}, price: {:?}",
            source.symbol,
            datetime,
            price
        );

        let index = match self.0.find_edge(
            source.graph_index.unwrap(),
            price.get_commodity().unwrap().graph_index.unwrap(),
        ) {
            Some(index) => index,
            None => {
                let dest = price.get_commodity().unwrap().graph_index.unwrap();
                self.add_edge(source.graph_index.unwrap(), dest, PriceMap::new())
            }
        };

        let prices = self.edge_weight_mut(index).unwrap();

        // Add the price to the price history.
        // prices.entry(key) ?
        prices.insert(datetime, price.quantity);
    }

    pub fn get_commodity(&self, index: NodeIndex) -> &Commodity {
        let ptr = self.node_weight(index).expect("index should be valid");
        unsafe { &**ptr }
    }

    // pub fn get_commodity_mut(&mut self, index: NodeIndex) -> &mut Commodity {
    //     let ptr = self.node_weight_mut(index).expect("index should be valid");
    //     unsafe { &mut ptr.read() }
    // }

    pub fn map_prices(&self) {
        todo!()
    }

    /// find_price(source, target, moment, oldest);
    pub fn find_price(
        &self,
        source_ptr: *const Commodity,
        target_ptr: *const Commodity,
        moment: NaiveDateTime,
        oldest: NaiveDateTime,
    ) -> Option<PricePoint> {
        assert_ne!(source_ptr, target_ptr);

        let source: CommodityIndex = commodity::from_ptr(source_ptr).graph_index.unwrap();
        let target: CommodityIndex = commodity::from_ptr(target_ptr).graph_index.unwrap();

        // Search for the shortest path using a*.
        let shortest_path = astar(&self.0, source, |finish| finish == target, |e| 1, |_| 0);
        if shortest_path.is_none() {
            return None;
        }

        // Get the price.
        let Some((distance, path)) = shortest_path else {
            panic!("should not happen")
        };

        log::debug!(
            "Shortest path found: hops={:?}, nodes={:?}",
            distance,
            &path
        );

        if distance == 1 {
            // direct link
            let Some((date, quantity)) = self.get_direct_price(source, target) else {
                panic!("should not happen!")
            };
            let pp = PricePoint::new(*date, Amount::new(*quantity, Some(target_ptr)));
            return Some(pp);
        } else {
            // else calculate the rate
            self.calculate_rate(source, target_ptr, path);
            todo!()
        }
    }

    fn calculate_rate(&self, source: CommodityIndex, target_ptr: *const Commodity, path: Vec<NodeIndex>) -> (NaiveDateTime, Quantity) {
        let mut result = Amount::new(Quantity::ONE, Some(target_ptr));
        let mut temp_source = source;
        for temp_target in path {
            // skip self
            if temp_target == temp_source {
                continue;
            }

            // get the price
            // TODO: include the datetime
            let (&temp_date, &temp_quantity) = self
                .get_direct_price(
                    temp_source,
                    temp_target,
                )
                .expect("price"); // , moment, oldest);

            // TODO: calculate the amount.
            result.quantity *= temp_quantity;
            // temp_price.when

            log::debug!("intermediate price from {:?} to {:?} = {:?} on {:?}", temp_source, temp_target, temp_quantity, temp_date);

            // source for the next leg
            temp_source = temp_target;
        }

        // TODO: What is the final date when multiple hops involved?
        //
        let when = Local::now().naive_local();

        // TODO: add to the price map.
        // self.add_price(commodity_index, datetime, price);

        // Some(PricePoint::new(when, result))
        (when, result.quantity)
    }

    /// Finds the price
    /// i.e. 1 EUR = 1.10 USD
    /// source: EUR
    /// target: USD
    pub fn get_direct_price(
        &self,
        source: CommodityIndex,
        target: CommodityIndex,
    ) -> Option<(&NaiveDateTime, &Quantity)> {
        let direct = self.find_edge(source, target);
        if let Some(edge_index) = direct {
            let price_history = self.edge_weight(edge_index).unwrap();
            if let Some(price_point) = get_latest_price(price_history) {
                Some(price_point)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn print_map(&self) {
        todo!()
    }
}

impl Deref for CommodityHistory {
    // specify the target type as i32
    type Target = Graph<*const Commodity, PriceMap>;

    // define the deref method that returns a reference to the inner value
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommodityHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Returns the latest (newest) price from the prices map.
///
/// BTree is doing all the work here, sorting the keys (dates).
fn get_latest_price(
    prices: &BTreeMap<NaiveDateTime, Quantity>,
) -> Option<(&NaiveDateTime, &Quantity)> {
    if prices.is_empty() {
        return None;
    }

    // let mut dates: Vec<&NaiveDateTime> = prices.keys().collect();
    // dates.sort();
    // let last_date = *dates.last().unwrap();
    // prices.get(last_date)

    // BTreeMap does this for us.
    prices.last_key_value()
}

/// Represents a price of a commodity.
/// i.e. (1) EUR = 1.20 AUD
///
/// TODO: Compare with price_point_t, which does not have the commodity_index,
/// if one type would be enough.
#[derive(Debug)]
pub struct Price {
    /// The commodity being priced.
    pub commodity: *const Commodity,
    /// Point in time at which the price is valid.
    pub datetime: NaiveDateTime,
    /// Price of the commodity. i.e. 1.20 AUD
    pub price: Amount,
}

impl Price {
    pub fn new(commodity: &Commodity, datetime: NaiveDateTime, cost: Amount) -> Self {
        Self {
            commodity: commodity as *const Commodity,
            datetime,
            price: cost,
        }
    }
    pub fn get_commodity(&self) -> &Commodity {
        unsafe { &*self.commodity }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use petgraph::stable_graph::NodeIndex;

    use super::{get_latest_price, CommodityHistory, PriceMap};
    use crate::{
        amount::{Amount, Quantity},
        commodity::{self, Commodity, PricePoint},
        journal::Journal,
        parser::{parse_amount, parse_datetime},
    };

    #[test]
    fn test_adding_commodity() {
        let mut hist = CommodityHistory::new();
        let c = Commodity::new("EUR");

        // Act
        let cdty_index = hist.add_commodity(&c);

        // Assert
        assert_eq!(1, hist.node_count());
        // TODO: assert_eq!("EUR", hist.node_weight(cdty_index).unwrap().symbol);
    }

    #[test]
    fn test_get_commodity() {
        let mut hist = CommodityHistory::new();
        let c = Commodity::new("EUR");
        let id = hist.add_commodity(&c);

        let actual = hist.get_commodity(id);

        assert_eq!("EUR", actual.symbol);
    }

    #[test]
    fn test_adding_price() {
        // Arrange
        let mut journal = Journal::new();
        let eur = journal.commodity_pool.create("EUR", None);
        let usd = journal.commodity_pool.create("USD", None);
        let local = Local::now();
        let today = local.naive_local();
        let price = Amount::new(25.into(), Some(usd));
        let hist = &mut journal.commodity_pool.commodity_history;

        // Act
        hist.add_price(eur, today, price);

        // Assert
        assert_eq!(2, hist.node_count());
        assert_eq!(1, hist.edge_count());

        let edge = hist.edge_weights().nth(0).unwrap();
        assert_eq!(&Quantity::from(25), edge.values().nth(0).unwrap());
    }

    #[test]
    fn test_index() {
        let mut graph = CommodityHistory::new();
        let eur = Commodity::new("EUR");
        let x = graph.add_commodity(&eur);
        let y = x.index();
        let z = NodeIndex::new(y);

        assert_eq!(z, x);
    }

    /// Gets the latest price.
    #[test]
    fn test_get_latest_price() {
        let mut prices = PriceMap::new();
        prices.insert(parse_datetime("2023-05-05").unwrap(), Quantity::from(10));
        prices.insert(parse_datetime("2023-05-01").unwrap(), Quantity::from(20));
        let newest_date = parse_datetime("2023-05-10").unwrap();
        prices.insert(newest_date, Quantity::from(30));
        prices.insert(parse_datetime("2023-05-02").unwrap(), Quantity::from(40));

        // act
        let Some((actual_date, actual_quantity)) = get_latest_price(&prices) else {
            panic!("Should not happen!")
        };

        // assert!(actual.is_some());
        assert_eq!(newest_date, *actual_date);
        assert_eq!(Quantity::from(30), *actual_quantity);
    }

    #[test]
    fn test_get_direct_price() {
        let journal = &mut Journal::new();
        let eur_ptr = journal.commodity_pool.create("EUR", None);
        let usd_ptr = journal.commodity_pool.create("USD", None);
        // add price
        let date = parse_datetime("2023-05-01").unwrap();
        let price = parse_amount("1.20 USD", journal).unwrap();
        assert!(!price.get_commodity().unwrap().symbol.is_empty());
        journal.commodity_pool.add_price(eur_ptr, date, price);

        // act
        let (actual_date, actual_quantity) = journal
            .commodity_pool
            .commodity_history
            .get_direct_price(
                commodity::from_ptr(eur_ptr).graph_index.unwrap(),
                commodity::from_ptr(usd_ptr).graph_index.unwrap(),
            )
            .unwrap();

        // assert
        // assert_eq!(eur_ptr, actual.price.commodity);
        // assert_eq!("2023-05-01 00:00:00", actual.datetime.to_string());
        // assert_eq!(actual.price.quantity, 1.20.into());
        assert_eq!(actual_date, &date);
        assert_eq!(Quantity::from("1.20"), *actual_quantity);
    }

    /// Test commodity exchange when there is a direct rate. EUR->USD
    #[test_log::test]
    fn test_find_price_1_hop() {
        let mut journal = Journal::new();
        // add commodities
        let eur_ptr = journal.commodity_pool.create("EUR", None);
        let usd_ptr = journal.commodity_pool.create("USD", None);
        // add price
        let date = parse_datetime("2023-05-01").unwrap();
        let price = parse_amount("1.20 USD", &mut journal).unwrap();
        let oldest = Local::now().naive_local();
        journal.commodity_pool.add_price(eur_ptr, date, price);

        // act
        let actual = journal
            .commodity_pool
            .commodity_history
            .find_price(eur_ptr, usd_ptr, date, oldest)
            .expect("price found");

        // assert
        assert_eq!(actual.when, date);
        assert_eq!(actual.price.quantity, "1.20".into());
        assert_eq!(
            actual.price.get_commodity().unwrap(),
            commodity::from_ptr(usd_ptr)
        );
    }

    #[test_log::test]
    fn test_calculate_rate() {
        // arrange
        let mut journal = Journal::new();
        let eur_ptr = journal.commodity_pool.create("EUR", None);
        let aud_ptr = journal.commodity_pool.create("AUD", None);
        let usd_ptr = journal.commodity_pool.create("USD", None);
        let source = commodity::from_ptr(eur_ptr).graph_index.unwrap();
        let path = vec![NodeIndex::new(0), NodeIndex::new(1), NodeIndex::new(2)];
        // prices
        let date = parse_datetime("2023-05-01").unwrap();
        // 1 EUR = 2 AUD
        let two_aud = parse_amount("2 AUD", &mut journal).unwrap();
        journal.commodity_pool.add_price(eur_ptr, date, two_aud);
        // 1 AUD = 3 USD
        let three_usd = parse_amount("3 USD", &mut journal).unwrap();
        journal.commodity_pool.add_price(aud_ptr, date, three_usd);

        // act
        let (actual_date, actual_quantity) = journal.commodity_pool.commodity_history.calculate_rate(source, usd_ptr, path);

        // assert
        assert_eq!(date, actual_date);
        assert_eq!(Quantity::from_str("3 USD").unwrap(), actual_quantity);
    }

    /// Test commodity exchange via an intermediary. EUR->AUD->USD
    #[test_log::test]
    fn test_find_price_2_hops() {
        let mut journal = Journal::new();
        let eur_ptr = journal.commodity_pool.create("EUR", None);
        let aud_ptr = journal.commodity_pool.create("AUD", None);
        let usd_ptr = journal.commodity_pool.create("USD", None);
        // prices
        let date = parse_datetime("2023-05-01").unwrap();
        // 1 EUR = 2 AUD
        let euraud = parse_amount("2 AUD", &mut journal).unwrap();
        journal.commodity_pool.add_price(eur_ptr, date, euraud);
        // 1 AUD = 3 USD
        let audusd = parse_amount("3 USD", &mut journal).unwrap();
        journal.commodity_pool.add_price(aud_ptr, date, audusd);
        let oldest = Local::now().naive_local();

        // act
        let actual = journal
            .commodity_pool
            .commodity_history
            .find_price(eur_ptr, usd_ptr, date, oldest)
            .unwrap();

        // assert
        assert_eq!(actual.when, date);
        // 1 EUR = 2 AUD = 6 USD
        assert_eq!(actual.price.quantity, 6.into());
        assert_eq!(actual.price.get_commodity().unwrap().symbol, "USD");
    }
}
