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
    commodity::{Commodity, PricePoint},
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
        let source: &Commodity;
        unsafe {
            source = &*source_ptr;
        }
        assert!(Some(source) != price.get_commodity());

        log::debug!(
            "adding price for {:?}, date: {:?}, price: {:?}",
            source,
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

        let source: CommodityIndex;
        let target: CommodityIndex;
        unsafe {
            source = (*source_ptr).graph_index.unwrap();
            target = (*target_ptr).graph_index.unwrap();
        }

        // Dijkstra returns a map of destination NodeId, path cost.
        // let shortest_paths = dijkstra(&self.graph, source, Some(target), |_| 1);
        // TODO: use the actual latest price value.
        let shortest_path = astar(
            &self.0,
            source,
            |finish| finish == target,
            |e| 1,
            |_| 0,
        );
        if shortest_path.is_none() {
            return None;
        }

        // Get the price.
        // let Some(distance) = shortest_paths.get(&target)
        // else {
        //     return None;
        // };
        // cost, path
        let Some((cost, path)) = shortest_path
            else { panic!("Check this case") };

        // if *distance == 1 {
        //     // direct link
        //     self.get_direct_price(source, target)
        // } else {
        //     // calculate the rate
        //     todo!()
        // }

        let mut result = Amount::new(Quantity::ONE, Some(target_ptr));
        let mut temp_source = source;
        for temp_target in path {
            // skip self
            if temp_target == temp_source {
                continue;
            }

            // get the price
            // TODO: include the datetime
            todo!("resolve this below");
            // let temp_price = self
            //     .get_direct_price(
            //         temp_source as *const Commodity,
            //         temp_target as *const Commodity,
            //     )
            //     .expect("price"); // , moment, oldest);

            // TODO: calculate the amount.
            // result *= temp_price.price;
            // temp_price.when

            // log::debug!("intermediate price from {:?} to {:?} = {:?}", temp_source, temp_target, temp_price);

            // source for the next leg
            todo!("fix below");
            // temp_source = temp_target;
        }

        // TODO: What is the final date when multiple hops involved?
        //
        let when = Local::now().naive_local();

        // TODO: add to the price map.
        // self.add_price(commodity_index, datetime, price)

        Some(PricePoint::new(when, result))
    }

    /// Finds the price
    /// i.e. 1 EUR = 1.10 USD
    /// source: EUR
    /// target: USD
    pub fn get_direct_price(
        &self,
        source_ptr: *const Commodity,
        target_ptr: *const Commodity,
    ) -> Option<PricePoint> {
        let source: CommodityIndex;
        let target: CommodityIndex;
        unsafe {
            source = (*source_ptr).graph_index.unwrap();
            target = (*target_ptr).graph_index.unwrap();
        }

        let direct = self.find_edge(source, target);
        if let Some(edge_index) = direct {
            let price_history = self.edge_weight(edge_index).unwrap();
            if let Some(mut price_point) = get_latest_price(price_history) {
                // price_point.price.get_commodity().unwrap().graph_index = Some(target);
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
fn get_latest_price(prices: &BTreeMap<NaiveDateTime, Quantity>) -> Option<PricePoint> {
    if prices.is_empty() {
        return None;
    }

    // let mut dates: Vec<&NaiveDateTime> = prices.keys().collect();
    // dates.sort();
    // let last_date = *dates.last().unwrap();
    // prices.get(last_date)

    // BTreeMap does this for us.
    if let Some((date, quantity)) = prices.last_key_value() {
        // Some(v)
        Some(PricePoint::new(*date, Amount::new(*quantity, None)))
    } else {
        None
    }
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
        commodity::{Commodity, PricePoint},
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
        let mut hist = CommodityHistory::new();
        let eur = Commodity::new("EUR");
        let eur_index = hist.add_commodity(&eur);
        let usd = Commodity::new("USD");
        let usd_index = hist.add_commodity(&usd);
        let local = Local::now();
        let today = local.naive_local();
        let price = Amount::new(25.into(), Some(&usd));

        // Act
        hist.add_price(&eur, today, price);

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
        let actual = get_latest_price(&prices);

        assert!(actual.is_some());
        assert_eq!(
            PricePoint::new(newest_date, Amount::new(Quantity::from(30), None)),
            actual.unwrap()
        );
    }

    #[test]
    fn test_get_direct_price() {
        let journal = &mut Journal::new();
        let eur_ptr = journal.commodity_pool.create("EUR", None);
        let usd_ptr = journal.commodity_pool.create("USD", None);
        // add price
        let date = parse_datetime("2023-05-01").unwrap();
        let price = parse_amount("1.20 USD", journal).unwrap();
        journal
            .commodity_pool
            .commodity_history
            .add_price(eur_ptr, date, price);

        // act
        let actual = journal
            .commodity_pool
            .commodity_history
            .get_direct_price(eur_ptr, usd_ptr)
            .unwrap();

        // assert
        // assert_eq!(eur_index, actual.commodity_index);
        // assert_eq!("2023-05-01 00:00:00", actual.datetime.to_string());
        // assert_eq!(actual.price.quantity, 1.20.into());
        assert_eq!(
            PricePoint::new(date, Amount::new(Quantity::from("1.20"), None)),
            actual
        );
    }

    #[test]
    fn test_find_price_1_hop() {
        let mut journal = Journal::new();
        // add commodities
        let eur = journal.commodity_pool.create("EUR", None);
        // let usd_index = journal.commodity_pool.create("USD", None);
        let usd = Commodity::new("USD");
        // add price
        let date = parse_datetime("2023-05-01").unwrap();
        let price = parse_amount("1.20 USD", &mut journal).unwrap();
        let oldest = Local::now().naive_local();
        journal.commodity_pool.add_price(eur, date, price);

        // act
        let actual = journal
            .commodity_pool
            .commodity_history
            .find_price(eur, &usd, date, oldest)
            .expect("price found");

        // assert
        // assert!(actual.is_some());
        // assert_eq!(eur_index, price.commodity_index.unwrap());
        assert_eq!(actual.when, date);
        assert_eq!(actual.price.quantity, "1.20".into());
        assert_eq!(actual.price.get_commodity(), Some(&usd));
    }

    #[test]
    fn test_find_price_2_hops() {
        let mut journal = Journal::new();
        let usd = Commodity::new("USD");
        let eur_index = journal.commodity_pool.create("EUR", None);
        let aud_index = journal.commodity_pool.create("AUD", None);
        let usd_index = journal.commodity_pool.create("USD", None);
        // prices
        let date = parse_datetime("2023-05-01").unwrap();
        // 1 EUR = 2 AUD
        let euraud = parse_amount("2 AUD", &mut journal).unwrap();
        journal.commodity_pool.add_price(eur_index, date, euraud);
        // 1 AUD = 3 USD
        let audusd = parse_amount("3 USD", &mut journal).unwrap();
        journal.commodity_pool.add_price(aud_index, date, audusd);
        let oldest = Local::now().naive_local();

        // act
        let actual = journal
            .commodity_pool
            .commodity_history
            .find_price(eur_index, usd_index, date, oldest)
            .unwrap();

        // assert
        // TODO: assert_eq!(actual.when, date);
        // 1 EUR = 2 AUD = 6 USD
        assert_eq!(actual.price.quantity, 6.into());
        assert_eq!(actual.price.get_commodity(), Some(&usd));
    }
}
