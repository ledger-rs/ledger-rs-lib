/*!
 * value.cc
 * 
 */

use chrono::NaiveDateTime;

use crate::{pool::{CommodityPool, CommodityIndex}, commodity::{Commodity, PricePoint}};

/// commodities = comma-separated list of symbols. Also can contain `=`.
/// Returns value_t.
pub(crate) fn exchange_commodities(commodities: &str, add_prices: bool, moment: &NaiveDateTime, pool: &mut CommodityPool) {
    if !commodities.contains(',') && !commodities.contains('=') {
        // only one commodity.
        return value(moment, pool.find_or_create(commodities, None), &pool);
    }

    todo!("complete")
}

fn value(moment: &NaiveDateTime, in_terms_of: Option<CommodityIndex>, pool: &CommodityPool) {
    // &Commodity

    amount_value(moment, in_terms_of, pool)

    // TODO: handle balance
}

/// amount.cc
/// optional<amount_t>
/// amount_t::value(const datetime_t&   moment,
///     const commodity_t * in_terms_of) const
fn amount_value(moment: &NaiveDateTime, in_terms_of: Option<CommodityIndex>, pool: &CommodityPool) {
    // if quantity 
    //   if has_commodity() && (in_terms_of || ! commodity().has_flags(COMMODITY_PRIMARY))
    let point: Option<PricePoint>;
    // let commodity

    // if has_annotation && annotation().price

    // if ! point
    // commodity().find_price(comm, moment)
}

#[cfg(test)]
mod tests {
    use crate::{journal::Journal, parse_file};

    #[test]
    fn test_exchange() {
        let mut journal = Journal::new();
        parse_file("tests/commodity_exchange.ledger", &mut journal);

        // act
        todo!("run bal -X USD")

        // assert

    }
}