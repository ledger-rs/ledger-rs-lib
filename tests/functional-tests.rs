/*!
 * Functional tests
 * 
 */

use std::io::Cursor;

use ledger_rs_lib::journal::{Journal, self};

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
    //j.
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