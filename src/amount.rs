use std::str::FromStr;

use crate::commodity::Commodity;
use rust_decimal::Decimal;

/**
 * Amount
 */

#[derive(Debug, PartialEq)]
pub struct Amount {
    pub quantity: Decimal,
    pub commodity: Option<Commodity>,
}

impl Amount {
    pub fn new(quantity: Decimal, commodity: Option<Commodity>) -> Self {
        Self { quantity, commodity }
    }

    pub fn null() -> Self {
        Self {
            quantity: Decimal::ZERO,
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
        if input.is_empty() {
            panic!("Invalid string for parsing into amount (empty)!")
        }

        // sequential parsing is probably better for handling all options.
        let first_char = input.chars().next().unwrap();
        if first_char == '-' || first_char.is_numeric() {
            // first_char == '.' || first_char == ',' || 
            // Starts with numeric.
            parse_number_first(input)
        } else {
            // symbol
            parse_symbol_first(input)
        }
    }
}

impl std::ops::Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        if self.commodity != rhs.commodity {
            panic!("don't know yet how to handle this")
        }

        let sum = self.quantity + rhs.quantity;

        Amount::new(sum, self.commodity)
    }
}

fn parse_quantity(input: &str) -> Decimal {
    // get rid of thousand separators
    // let clean = input.replace(',', '');

    Decimal::from_str(input).unwrap()
    
    // Decimal::from_str_radix(input, 10).expect("amount parsed")
}

fn parse_number_first(input: &str) -> Amount {
    // find the separation index
    let mut separator_index: usize = input.len();
    for (i, c) in input.char_indices() {
        if c == '-' || c == ',' || c == '.' || c.is_numeric() {
            // skip
        } else {
            separator_index = i;
            break;
        }
    }
    let quantity_str = &input[..separator_index];
    let symbol_str = &input[separator_index..];
    
    let quantity = parse_quantity(quantity_str);
    let commodity = parse_symbol(symbol_str);

    Amount { quantity, commodity }
}

fn parse_symbol_first(input: &str) -> Amount {
    // find the separation index
    let mut separator_index: usize = input.len();
    for (i, c) in input.char_indices() {
        if c == '-' || c == ',' || c == '.' || c.is_numeric() {
            separator_index = i;
            break;
        } else {
            // skip
        }
    }

    let symbol_str = &input[..separator_index];
    let quantity_str = &input[separator_index..];
    
    let quantity = parse_quantity(quantity_str);
    let commodity = parse_symbol(symbol_str);

    Amount { quantity, commodity }
}

fn parse_symbol(input: &str) -> Option<Commodity> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    } else {
        return Some(Commodity::new(trimmed));
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use rust_decimal_macros::dec;

    use crate::commodity::Commodity;

    use super::{Amount, parse_quantity};

    #[test]
    fn test_positive_no_commodity() {
        let expected = Amount {
            quantity: Decimal::from_i16(20).expect("20"),
            commodity: None,
        };
        let actual = Amount::parse("20");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_no_commodity() {
        let actual = Amount::parse("-20");
        let expected = Amount {
            quantity: Decimal::from_i16(-20).expect("what can go wrong here?"),
            commodity: None,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pos_w_commodity_separated() {
        let actual = Amount::parse("20 EUR");
        let expected = Amount {
            quantity: Decimal::from_i16(20).expect("what can go wrong here?"),
            commodity: Some(Commodity::new("EUR")),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_neg_commodity_separated() {
        let actual = Amount::parse("-20 EUR");
        let expected = Amount {
            quantity: Decimal::from_i16(-20).expect("what can go wrong here?"),
            commodity: Some(Commodity::new("EUR")),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_full_w_commodity_separated() {
        let expected = Amount {
            quantity: Decimal::from_i16(-20000).expect("what can go wrong here?"),
            commodity: Some(Commodity::new("EUR")),
        };

        let actual = Amount::parse("-20000.00 EUR");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_full_commodity_first() {
        let expected = Amount {
            quantity: Decimal::from_i16(-20000).expect("what can go wrong here?"),
            commodity: Some(Commodity::new("A$")),
        };

        let actual = Amount::parse("A$-20000.00");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_quantity_separators() {
        let input = "-1000000.00";
        let expected = Decimal::from_i32(-1_000_000).unwrap();
        let actual = parse_quantity(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_addition() {
        let c1 = Commodity::new("EUR");
        let left = Amount::new(dec!(10), Some(c1));
        let c2 = Commodity::new("EUR");
        let right = Amount::new(dec!(15), Some(c2));

        let actual = left + right;

        assert_eq!(dec!(25), actual.quantity);
        assert!(actual.commodity.is_some());
        assert_eq!("EUR", actual.commodity.unwrap().symbol);
    }
}
