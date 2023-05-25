/**
 * balance.h + .cc
 *
 * Intended to help with storing amounts in multiple commodities.
 */
use std::collections::HashMap;

use crate::amount::Amount;
use crate::commodity::Commodity;

/// Balance
pub(crate) struct Balance {
    /// Map of commodity index, Amount
    pub amounts: HashMap<usize, Amount>, // try Symbol/Amount for easier search.
                                     // amounts: HashMap<String, Amount>
}

impl Balance {
    pub fn new() -> Self {
        // Add null commodity
        // let mut amounts: HashMap<usize, Amount> = HashMap::new();
        // amounts.insert("", Amount::new(0, None));

        Self {
            amounts: HashMap::new(),
        }
    }

    pub fn add(&mut self, amount: &Amount) {
        // separate amount for each commodity
        let commodity_index = match amount.commodity_index {
            Some(commodity_index) => commodity_index,
            // using max for null value. TODO: find a better way.
            None => usize::MAX,
        };

        match self.amounts.contains_key(&commodity_index) {
            true => {
                // add to existing commodity
                let Some(mut existing_amount) = self.amounts.get_mut(&commodity_index)
                    else {panic!("should not happen")};
                existing_amount.add(&amount);
            }
            false => {
                // Add new commodity amount
                self.amounts
                    .insert(commodity_index, Amount::copy_from(amount));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::Balance;
    use crate::amount::Amount;

    #[test]
    fn test_adding_first_amount_no_commodity() {
        let amount = Amount::new(dec!(25), None);
        let mut balance = Balance::new();

        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(1, balance.amounts.len());
        assert_eq!(dec!(25), balance.amounts.iter().next().unwrap().1.quantity);
        assert_eq!(None, balance.amounts.iter().next().unwrap().1.commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_no_commodity() {
        let mut balance = Balance::new();
        
        // Act
        let amount = Amount::new(dec!(25), None);
        balance.add(&amount);

        let amount = Amount::new(dec!(5), None);
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(1, balance.amounts.len());
        assert_eq!(dec!(30), balance.amounts.iter().next().unwrap().1.quantity);
        assert_eq!(None, balance.amounts.iter().next().unwrap().1.commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_with_commodities() {
        let mut balance = Balance::new();
        
        // Act
        let amount = Amount::new(dec!(25), Some(0));
        balance.add(&amount);

        let amount = Amount::new(dec!(5), None);
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(2, balance.amounts.len());
        assert_eq!(dec!(25), balance.amounts.iter().nth(0).unwrap().1.quantity);
        assert_eq!(Some(0), balance.amounts.iter().nth(0).unwrap().1.commodity_index);

        assert_eq!(dec!(5), balance.amounts.iter().nth(1).unwrap().1.quantity);
        assert_eq!(None, balance.amounts.iter().nth(1).unwrap().1.commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_with_some_commodities() {
        let mut balance = Balance::new();
        
        // Act
        let amount = Amount::new(dec!(25), Some(0));
        balance.add(&amount);

        let amount = Amount::new(dec!(5), Some(1));
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(2, balance.amounts.len());

        assert_eq!(dec!(25), balance.amounts.iter().nth(0).unwrap().1.quantity);
        assert_eq!(Some(0), balance.amounts.iter().nth(0).unwrap().1.commodity_index);

        assert_eq!(dec!(5), balance.amounts.iter().nth(1).unwrap().1.quantity);
        assert_eq!(Some(1), balance.amounts.iter().nth(1).unwrap().1.commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_with_same_commodity() {
        let mut balance = Balance::new();
        
        // Act
        let amount = Amount::new(dec!(25), Some(0));
        balance.add(&amount);

        let amount = Amount::new(dec!(5), Some(0));
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(1, balance.amounts.len());
        
        assert_eq!(dec!(30), balance.amounts.iter().nth(0).unwrap().1.quantity);
        assert_eq!(Some(0), balance.amounts.iter().nth(0).unwrap().1.commodity_index);
    }

}
