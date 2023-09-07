/*!
 * Functional tests
 * 
 */

use std::io::Cursor;

use chrono::Local;
use ledger_rs_lib::{journal::Journal, amount::{Amount, Quantity}};

/// TODO: complete the functionality and the test
//#[test]
fn test_price_reading() {
    // Arrange
    let mut j = Journal::new();
    let text = r#"
P 2022-03-03 13:00:00 EUR 1.12 USD
"#;
    // Act
    ledger_rs_lib::parse_text(text, &mut j);

    // Assert
    let eur = j.commodity_pool.find("EUR").unwrap();
    let usd = j.commodity_pool.find("USD").unwrap();
    let three_eur = Amount::new(Quantity::from(3), Some(eur));
    let exch_rate = Amount::new(Quantity::from(1.5), Some(usd));

    let (cost_breakdown, price) = j.commodity_pool.exchange(&three_eur, &exch_rate, true, Local::now().naive_local());
    assert!(price.is_some());
    todo!("check that the price was parsed")
}

fn test_commodity_conversion_with_price() {
    let input = r#"
P 2022-03-03 13:00:00 EUR 1.12 USD

2023-01-10 Vacation
    Expenses:Vacation  20 EUR
    Assets:Cash
"#;
    let mut journal = Journal::new();
    journal.read(Cursor::new(input));

    todo!("run a report with -X USD")

    // assert
    
}