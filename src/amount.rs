use std::{
    ops::{AddAssign, Mul},
    str::FromStr,
};

use crate::commodity::Commodity;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

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
        Self {
            quantity,
            commodity,
        }
    }

    pub fn null() -> Self {
        Self {
            quantity: dec!(0),
            commodity: None,
        }
    }

    /// Parses the amount from string.
    /// Currently just accept a simple format "[-]NUM[ SYM]"
    ///
    /// Acceptable formats should be like in Ledger:
    ///   [-]NUM[ ]SYM [@ AMOUNT]
    ///   SYM[ ][-]NUM [@ AMOUNT]
    pub(crate) fn parse(input: &str) -> Option<Amount> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return None;
        }

        // sequential parsing is probably better for handling all options.
        let first_char = trimmed.chars().next().unwrap();
        if first_char == '-' || first_char.is_numeric() {
            // first_char == '.' || first_char == ',' ||
            // Starts with numeric.
            parse_number_first(trimmed)
        } else {
            // symbol
            parse_symbol_first(trimmed)
        }
    }

    pub fn add(&mut self, other: &Amount) {
        if self.commodity != other.commodity {
            panic!("don't know yet how to handle this")
        }
        if other.quantity.is_zero() {
            // nothing to do
            return;
        }

        self.quantity += other.quantity;
    }

    /// Returns an inverse amount.
    /// Normally it is a quantity with the opposite sign.
    pub fn inverse(&self) -> Amount {
        let new_quantity = self.quantity.mul(dec!(-1));
        let new_commodity = match &self.commodity {
            Some(c) => Some(Commodity::new(&c.symbol)),
            None => None,
        };
        Amount::new(new_quantity, new_commodity)
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

impl AddAssign<Amount> for Amount {
    fn add_assign(&mut self, other: Amount) {
        if self.commodity != other.commodity {
            panic!("don't know yet how to handle this")
        }

        self.quantity += other.quantity;
    }
}

fn parse_quantity(input: &str) -> Option<Decimal> {
    // handle empty string
    if input.is_empty() {
        return None;
    }

    // get rid of thousand separators
    // let clean = input.replace(',', '');

    Some(Decimal::from_str(input).unwrap())

    // Decimal::from_str_radix(input, 10).expect("amount parsed")
}

fn parse_number_first(input: &str) -> Option<Amount> {
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

    if quantity.is_some() {
        return Some(Amount::new(quantity.unwrap(), commodity));
    } else {
        return None;
    }
}

fn parse_symbol_first(input: &str) -> Option<Amount> {
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

    if quantity.is_some() {
        return Some(Amount::new(quantity.unwrap(), commodity));
    } else {
        return None;
    }
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
    use rust_decimal_macros::dec;

    use crate::commodity::Commodity;

    use super::{parse_quantity, Amount};

    #[test]
    fn test_positive_no_commodity() {
        let expected = Amount {
            quantity: dec!(20),
            commodity: None,
        };
        let actual = Amount::parse("20").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_negative_no_commodity() {
        let actual = Amount::parse("-20").unwrap();
        let expected = Amount {
            quantity: dec!(-20),
            commodity: None,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pos_w_commodity_separated() {
        let actual = Amount::parse("20 EUR").unwrap();
        let expected = Amount {
            quantity: dec!(20),
            commodity: Some(Commodity::new("EUR")),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_neg_commodity_separated() {
        let actual = Amount::parse("-20 EUR").unwrap();
        let expected = Amount {
            quantity: dec!(-20),
            commodity: Some(Commodity::new("EUR")),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_full_w_commodity_separated() {
        let expected = Amount {
            quantity: dec!(-20000),
            commodity: Some(Commodity::new("EUR")),
        };

        let actual = Amount::parse("-20000.00 EUR").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_full_commodity_first() {
        let expected = Amount {
            quantity: dec!(-20000),
            commodity: Some(Commodity::new("A$")),
        };

        let actual = Amount::parse("A$-20000.00").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_quantity_separators() {
        let input = "-1000000.00";
        let expected = Some(dec!(-1_000_000));
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

    #[test]
    fn test_add_assign() {
        let c1 = Commodity::new("EUR");
        let mut actual = Amount::new(dec!(21), Some(c1));
        let c2 = Commodity::new("EUR");
        let other = Amount::new(dec!(13), Some(c2));

        // actual += addition;
        actual.add(&other);

        assert_eq!(dec!(34), actual.quantity);
    }

    #[test]
    fn test_null_amount() {
        let input = " ";
        let actual = Amount::parse(input);

        assert_eq!(None, actual);
    }
}
