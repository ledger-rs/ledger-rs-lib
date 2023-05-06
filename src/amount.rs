use crate::commodity::Commodity;
use rust_decimal::Decimal;

/**
 * Amount
 */

 #[derive(Debug, PartialEq)]
pub struct Amount {
    value: Decimal,
    commodity: Option<Commodity>,
}

impl Amount {
    pub fn new() -> Self {
        Self {
            value: Decimal::ZERO,
            commodity: None,
        }
    }

    /// Parses the amount from string.
    /// Currently just accept a simple format "[-]NUM[ SYM]"
    /// 
    /// Acceptable formats should be like in Ledger:
    ///   [-]NUM[ ]SYM [@ AMOUNT]
    ///   SYM[ ][-]NUM [@ AMOUNT]
    pub(crate) fn parse(input: &str) -> Amount {
        let has_number = input.chars().any(|c| c.is_numeric());
        if !has_number {
            panic!("No numeric value found in the string to parse into amount.");
        }

        let sep_option = input.find(' ');
        let amount = match sep_option {
            Some(index) => parse_symbol_separated(input, index),
            None => parse_numeric_only(input),
        };

        amount
    }
}

fn parse_numeric_only(input: &str) -> Amount {
    let value = Decimal::from_str_radix(input, 10).expect("amount parsed");
    Amount { value, commodity: None }
}

fn parse_symbol_separated(input: &str, sep_index: usize) -> Amount {
    //let separator_index = .expect("separator found");

    let val_str = &input[..sep_index];
    let symbol_str = &input[sep_index+1..];
    
    let value = Decimal::from_str_radix(val_str, 10).expect("amount parsed");

    let commodity: Option<Commodity> = Some(Commodity::new(symbol_str));

    Amount { value, commodity }
}

/// Identifies the quantity in the input string.
/// Returns the str of the quantity value and
/// the last position index (for cursor control).
/// Parameters:
/// input
fn parse_quantity(input: &str) -> (&str, usize) {
    let mut start: usize = 0;
    let mut end: usize = 0;

    if input.chars().next() == Some('-') {
        start += 1;
    }

    // read characters so long as they are numeric.
    for (i, c) in input.char_indices().skip(start) {
        if c.is_digit(10) || c == '.' || c == ',' {
            continue;
        } else {
            return (&input[start..i], i);
        }
    }
    return ("", 0);
}

#[cfg(test)]
mod tests {
    use rust_decimal::{prelude::FromPrimitive, Decimal};

    use crate::commodity::Commodity;

    use super::Amount;

    #[test]
    fn test_positive_no_commodity() {
        let actual = Amount::parse("20");
        let expected = Amount {
            value: Decimal::from_i16(20).expect("20"),
            commodity: None,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_no_commodity() {
        let actual = Amount::parse("-20");
        let expected = Amount {
            value: Decimal::from_i16(-20).expect("what can go wrong here?"),
            commodity: None,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pos_w_commodity_separated() {
        let actual = Amount::parse("20 EUR");
        let expected = Amount {
            value: Decimal::from_i16(20).expect("what can go wrong here?"),
            commodity: Some(Commodity::new("EUR")),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_neg_commodity_separated() {
        let actual = Amount::parse("-20 EUR");
        let expected = Amount {
            value: Decimal::from_i16(-20).expect("what can go wrong here?"),
            commodity: Some(Commodity::new("EUR")),
        };

        assert_eq!(expected, actual);
    }
}
