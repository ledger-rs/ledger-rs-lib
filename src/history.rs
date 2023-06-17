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

use std::collections::{BTreeMap, HashMap};

use chrono::{NaiveDateTime, Local, NaiveTime};
use petgraph::{
    algo::{astar, dijkstra},
    stable_graph::NodeIndex,
    Graph,
};

use crate::{
    amount::{Amount, Decimal},
    commodity::{Commodity, PricePoint},
    pool::CommodityIndex,
};

// type PriceMap = HashMap<NaiveDateTime, Amount>;
type PriceMap = BTreeMap<NaiveDateTime, Decimal>;
// type PriceMap = Rc<RefCell<BTreeMap<NaiveDateTime, Decimal>>>;

// #[derive(Clone, Copy)]
// pub(crate) struct PriceMap(BTreeMap<NaiveDateTime, Decimal>);

pub(crate) struct CommodityHistory {
    pub(crate) graph: Graph<Commodity, PriceMap>,
}

// pub(crate) type CommodityHistory = Graph<Commodity, PriceMap>;

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
        // prices.entry(key) ?
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
        oldest: NaiveDateTime,
    ) -> Option<PricePoint> {
        assert_ne!(source, target);

        // Dijkstra returns a map of destination NodeId, path cost.
        // let shortest_paths = dijkstra(&self.graph, source, Some(target), |_| 1);
        // TODO: use the actual latest price value.
        let shortest_path = astar(&self.graph, source, |finish| finish == target, |e| 1, |_| 0);
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

        let mut result = Amount::new(Decimal::ONE, Some(target));
        let mut temp_source = source;
        for temp_target in path {
            // skip self
            if temp_target == temp_source {
                continue;
            }

            // get the price
            // TODO: include the datetime
            let temp_price = self.get_direct_price(temp_source, temp_target)
                .expect("price"); // , moment, oldest);

            // TODO: calculate the amount.
            result *= temp_price.price;
            // temp_price.when

            // log::debug!("intermediate price from {:?} to {:?} = {:?}", temp_source, temp_target, temp_price);

            // source for the next leg
            temp_source = temp_target;
        }

        // TODO: What is the final date when multiple hops involved?
        // 
        let when = Local::now().date_naive().and_time(NaiveTime::MIN);

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
        source: CommodityIndex,
        target: CommodityIndex,
    ) -> Option<PricePoint> {
        let direct = self.graph.find_edge(source, target);
        if let Some(edge_index) = direct {
            let price_history = self.graph.edge_weight(edge_index).unwrap();
            if let Some(mut price) = get_latest_price(price_history) {
                price.price.commodity_index = Some(target);
                Some(price)
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

/// Returns the latest (newest) price from the prices map.
///
/// BTree is doing all the work here, sorting the keys (dates).
fn get_latest_price(prices: &BTreeMap<NaiveDateTime, Decimal>) -> Option<PricePoint> {
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
    pub commodity_index: CommodityIndex,
    /// Point in time at which the price is valid.
    pub datetime: NaiveDateTime,
    /// Price of the commodity. i.e. 1.20 AUD
    pub price: Amount,
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveTime};
    use petgraph::stable_graph::NodeIndex;

    use super::{get_latest_price, CommodityHistory, PriceMap};
    use crate::{
        amount::{Amount, Decimal},
        commodity::{Commodity, PricePoint},
        journal::Journal,
        parse_file,
        parser::{parse_amount, parse_date, parse_datetime},
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

    /// Gets the latest price.
    #[test]
    fn test_get_latest_price() {
        let mut prices = PriceMap::new();
        prices.insert(parse_datetime("2023-05-05").unwrap(), Decimal::from(10));
        prices.insert(parse_datetime("2023-05-01").unwrap(), Decimal::from(20));
        let newest_date = parse_datetime("2023-05-10").unwrap();
        prices.insert(newest_date, Decimal::from(30));
        prices.insert(parse_datetime("2023-05-02").unwrap(), Decimal::from(40));

        // act
        let actual = get_latest_price(&prices);

        assert!(actual.is_some());
        assert_eq!(
            PricePoint::new(newest_date, Amount::new(Decimal::from(30), None)),
            actual.unwrap()
        );
    }

    #[test]
    fn test_get_direct_price() {
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
        let actual = journal
            .commodity_pool
            .commodity_history
            .get_direct_price(eur_index, usd_index)
            .unwrap();

        // assert
        // assert_eq!(eur_index, actual.commodity_index);
        // assert_eq!("2023-05-01 00:00:00", actual.datetime.to_string());
        // assert_eq!(actual.price.quantity, 1.20.into());
        assert_eq!(
            PricePoint::new(date, Amount::new(Decimal::from("1.20"), Some(usd_index))),
            actual
        );
    }

    #[test]
    fn test_find_price_1_hop() {
        let mut journal = Journal::new();
        // add commodities
        let eur_index = journal.commodity_pool.create("EUR", None);
        let usd_index = journal.commodity_pool.create("USD", None);
        // add price
        let date = parse_datetime("2023-05-01").unwrap();
        let price = parse_amount("1.20 USD", &mut journal).unwrap();
        let oldest = Local::now().date_naive().and_time(NaiveTime::MIN);
        journal.commodity_pool.add_price(eur_index, date, price);

        // act
        let actual = journal
            .commodity_pool
            .commodity_history
            .find_price(eur_index, usd_index, date, oldest)
            .expect("price found");

        // assert
        // assert!(actual.is_some());
        // assert_eq!(eur_index, price.commodity_index.unwrap());
        assert_eq!(actual.when, date);
        assert_eq!(actual.price.quantity, "1.20".into());
        assert_eq!(actual.price.commodity_index, Some(usd_index));
    }

    #[test]
    fn test_find_price_2_hops() {
        let mut journal = Journal::new();
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
        assert_eq!(actual.price.commodity_index, Some(usd_index));
    }
}
