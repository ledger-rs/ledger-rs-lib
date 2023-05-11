/**
 * Commodity Pool
 */

use std::collections::HashMap;

use crate::commodity::Commodity;

pub(crate) struct CommodityPool {
    commodities: HashMap<String, Commodity>,
    // annotated_commodities
    // commodity_price_history
    // null_commodity
    // default_commodity
}

impl CommodityPool {
    pub fn create() -> Commodity {
        todo!()

        // add price history
    }
}

#[cfg(test)]
mod tests {

}