/*!
 * Stores the balance
 * 
 * balance.h + .cc
 *
 * Intended to help with storing amounts in multiple commodities.
 */

 use std::ops::{SubAssign, AddAssign};
 
 use crate::amount::Amount;

/// Balance
#[derive(Debug)]
pub struct Balance {
    /// Map of commodity index, Amount
    // pub amounts: HashMap<CommodityIndex, Amount>, // try Symbol/Amount for easier search.
    // amounts: HashMap<String, Amount>

    /// Amounts, in different currencies.
    /// The currency information is contained within the Amount instance.
    pub amounts: Vec<Amount>,
}

impl Balance {
    pub fn new() -> Self {
        // Add null commodity
        // let mut amounts: HashMap<usize, Amount> = HashMap::new();
        // amounts.insert("", Amount::new(0, None));

        Self {
            // amounts: HashMap::new(),
            amounts: vec![],
        }
    }

    /// Add an Amount to the Balance.
    /// If an amount in the same commodity is found, it is added,
    /// otherwise, a new Amount is created.
    pub fn add(&mut self, amount: &Amount) {
        match self
            .amounts
            .iter_mut()
            .find(|amt| amt.commodity_index == amount.commodity_index)
        {
            Some(existing_amount) => {
                // append to the amount
                existing_amount.add(&amount);
            }
            None => {
                // Balance not found for the commodity. Create new.
                self.amounts.push(Amount::copy_from(amount));
            }
        };
    }
}

impl SubAssign<Amount> for Balance {
    fn sub_assign(&mut self, amount: Amount) {
        match self
            .amounts
            .iter_mut()
            .find(|amt| amt.commodity_index == amount.commodity_index)
        {
            Some(existing_amount) => {
                // append to the amount
                *existing_amount -= amount;
            }
            None => {
                // Balance not found for the commodity. Create new.
                self.amounts.push(Amount::copy_from(&amount.inverse()));
            }
        };
    }
}

impl AddAssign<Amount> for Balance {
    fn add_assign(&mut self, other: Amount) {
        self.add(&other);
    }
}

#[cfg(test)]
mod tests {
    use super::Balance;
    use crate::{amount::{Amount, Decimal}, pool::CommodityIndex};

    #[test]
    fn test_adding_first_amount_no_commodity() {
        let amount = Amount::new(25.into(), None);
        let mut balance = Balance::new();

        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(1, balance.amounts.len());
        assert_eq!(Decimal::from(25), balance.amounts.iter().next().unwrap().quantity);
        assert_eq!(None, balance.amounts.iter().next().unwrap().commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_no_commodity() {
        let mut balance = Balance::new();

        // Act
        let amount = Amount::new(25.into(), None);
        balance.add(&amount);

        let amount = Amount::new(5.into(), None);
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(1, balance.amounts.len());
        assert_eq!(Decimal::from(30), balance.amounts.iter().next().unwrap().quantity);
        assert_eq!(None, balance.amounts.iter().next().unwrap().commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_with_commodities() {
        let mut balance = Balance::new();

        // Act
        let amount = Amount::new(25.into(), Some(CommodityIndex::new(0)));
        balance.add(&amount);

        let amount = Amount::new(5.into(), None);
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(2, balance.amounts.len());
        assert_eq!(Decimal::from(25), balance.amounts.iter().nth(0).unwrap().quantity);
        assert_eq!(
            Some(CommodityIndex::new(0)),
            balance.amounts.iter().nth(0).unwrap().commodity_index
        );

        assert_eq!(Decimal::from(5), balance.amounts.iter().nth(1).unwrap().quantity);
        assert_eq!(None, balance.amounts.iter().nth(1).unwrap().commodity_index);
    }

    #[test]
    fn test_adding_two_amounts_with_some_commodities() {
        let mut balance = Balance::new();

        // Act
        let amount = Amount::new(25.into(), Some(CommodityIndex::new(0)));
        balance.add(&amount);

        let amount = Amount::new(5.into(), Some(CommodityIndex::new(1)));
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(2, balance.amounts.len());

        assert_eq!(Decimal::from(25), balance.amounts.iter().nth(0).unwrap().quantity);
        assert_eq!(
            Some(CommodityIndex::new(0)),
            balance.amounts.iter().nth(0).unwrap().commodity_index
        );

        assert_eq!(Decimal::from(5), balance.amounts.iter().nth(1).unwrap().quantity);
        assert_eq!(
            Some(CommodityIndex::new(1)),
            balance.amounts.iter().nth(1).unwrap().commodity_index
        );
    }

    #[test]
    fn test_adding_two_amounts_with_same_commodity() {
        let mut balance = Balance::new();

        // Act
        let amount = Amount::new(25.into(), Some(CommodityIndex::new(0)));
        balance.add(&amount);

        let amount = Amount::new(5.into(), Some(CommodityIndex::new(0)));
        balance.add(&amount);

        // Assert
        assert!(!balance.amounts.is_empty());
        assert_eq!(1, balance.amounts.len());

        assert_eq!(Decimal::from(30), balance.amounts.iter().nth(0).unwrap().quantity);
        assert_eq!(
            Some(CommodityIndex::new(0)),
            balance.amounts.iter().nth(0).unwrap().commodity_index
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut bal = Balance::new();
        let amount = Amount::new(10.into(), Some(0.into()));
        let expected = amount.inverse();

        bal -= amount;

        assert_eq!(1, bal.amounts.len());
        assert_eq!(expected, bal.amounts[0]);
    }
}
