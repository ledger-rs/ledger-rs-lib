/*!
 * Amount and the Decimal numeric type
 */

use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign},
};

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

use crate::pool::CommodityIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Amount {
    pub quantity: Decimal,
    pub commodity_index: Option<CommodityIndex>,
}

impl Amount {
    pub fn new(quantity: Decimal, commodity_index: Option<CommodityIndex>) -> Self {
        Self {
            quantity,
            commodity_index,
        }
    }

    /// Returns an absolute (positive) Amount.
    pub fn abs(&self) -> Amount {
        let mut result = self.clone();
        result.quantity.set_sign_positive();
        result
    }

    /// Creates a new Amount instance.
    /// Parses the quantity only and uses the given commodity index.
    pub fn parse(quantity: &str, commodity_index: Option<CommodityIndex>) -> Option<Self> {
        if quantity.is_empty() {
            return None;
        }

        let quantity_result = Decimal::from_str(quantity);
        if quantity_result.is_err() {
            return None;
        }

        let amount = Self {
            quantity: quantity_result.unwrap(),
            commodity_index,
        };

        Some(amount)
    }

    pub fn copy_from(other: &Amount) -> Self {
        // let com = match &other.commodity {
        //     Some(other_commodity) => {
        //         //let symbol = &other.commodity.as_ref().unwrap().symbol;
        //         let s = &other_commodity.symbol;
        //         let c = Commodity::new(s);
        //         Some(c)
        //     }
        //     None => None,
        // };

        Self {
            quantity: other.quantity,
            commodity_index: other.commodity_index,
        }
    }

    pub fn null() -> Self {
        Self {
            quantity: 0.into(),
            commodity_index: None,
        }
    }

    pub fn add(&mut self, other: &Amount) {
        if self.commodity_index != other.commodity_index {
            log::error!("different commodities");
            panic!("don't know yet how to handle this")
        }
        if other.quantity.is_zero() {
            // nothing to do
            return;
        }

        self.quantity += other.quantity;
    }

    /// Creates an amount with the opposite sign on the quantity.
    pub fn inverse(&self) -> Amount {
        let new_quantity = if self.quantity.is_sign_positive() {
            let mut x = self.quantity.clone();
            x.set_sign_negative();
            x
        } else {
            self.quantity
        };

        Amount::new(new_quantity, self.commodity_index)
    }

    /// Inverts the sign on the amount.
    pub fn invert(&mut self) {
        if self.quantity.is_sign_positive() {
            self.quantity.set_sign_negative();
        } else {
            self.quantity.set_sign_positive();
        }
    }

    /// Indicates whether the amount is initialized.
    /// This is a 0 quantity and no Commodity.
    pub fn is_null(&self) -> bool {
        if self.quantity.is_zero() {
            return self.commodity_index.is_none();
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        self.quantity.is_zero()
    }
}

impl std::ops::Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        if self.commodity_index != rhs.commodity_index {
            panic!("don't know yet how to handle this")
        }

        let sum = self.quantity + rhs.quantity;

        Amount::new(sum, self.commodity_index)
    }
}

impl AddAssign<Amount> for Amount {
    fn add_assign(&mut self, other: Amount) {
        if self.commodity_index != other.commodity_index {
            panic!("don't know yet how to handle this")
        }

        self.quantity += other.quantity;
    }
}

impl Div for Amount {
    type Output = Amount;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result = Amount::new(0.into(), None);

        if self.commodity_index.is_none() {
            result.commodity_index = rhs.commodity_index;
        } else {
            result.commodity_index = self.commodity_index
        }

        result.quantity = self.quantity / rhs.quantity;

        result
    }
}

impl Mul<Amount> for Amount {
    type Output = Amount;

    fn mul(self, other: Amount) -> Amount {
        let quantity = self.quantity * other.quantity;

        let commodity_index = if self.commodity_index.is_none() {
            other.commodity_index
        } else {
            self.commodity_index
        };

        Amount::new(quantity, commodity_index)
    }
}

impl From<i32> for Amount {
    fn from(value: i32) -> Self {
        Amount::new(Decimal::from(value), None)
    }
}

impl SubAssign<Amount> for Amount {
    fn sub_assign(&mut self, other: Amount) {
        if self.commodity_index != other.commodity_index {
            panic!("The commodities do not match");
        }

        self.quantity -= other.quantity;
    }
}

impl MulAssign<Amount> for Amount {
    fn mul_assign(&mut self, rhs: Amount) {
        // multiply the quantity
        self.quantity *= rhs.quantity;

        // get the other commodity, if we don't have one.
        if self.commodity_index.is_none() && rhs.commodity_index.is_some() {
            self.commodity_index = rhs.commodity_index;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decimal(rust_decimal::Decimal);

const ZERO: Decimal = Decimal(rust_decimal::Decimal::ZERO);

impl Decimal {
    pub const ZERO: Decimal = ZERO;

    pub fn from_str(str: &str) -> Result<Self, anyhow::Error> {
        Ok(Self(rust_decimal::Decimal::from_str_exact(str)?))
    }

    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn set_sign_negative(&mut self) {
        self.0.set_sign_negative(true)
    }

    pub fn set_sign_positive(&mut self) {
        self.0.set_sign_positive(true)
    }
}

impl From<i32> for Decimal {
    fn from(value: i32) -> Self {
        Decimal(rust_decimal::Decimal::from(value))
    }
}

impl From<f32> for Decimal {
    fn from(value: f32) -> Self {
        Decimal(rust_decimal::Decimal::from_f32(value).unwrap())
    }
}

/// Creates a Decimal value from a string. Panics if invalid.
impl From<&str> for Decimal {
    fn from(value: &str) -> Self {
        Self(rust_decimal::Decimal::from_str_exact(value).unwrap())
        // Decimal::from_str(value).unwrap()
    }
}

impl Into<i32> for Decimal {
    fn into(self) -> i32 {
        self.0.to_i32().unwrap()
    }
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, other: Decimal) -> Decimal {
        Decimal(self.0 + other.0)
    }
}

impl AddAssign<Decimal> for Decimal {
    fn add_assign(&mut self, other: Decimal) {
        self.0 += other.0;
    }
}

impl Div<Decimal> for Decimal {
    type Output = Decimal;

    fn div(self, other: Decimal) -> Decimal {
        Self(self.0.div(other.0))
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, other: Decimal) -> Decimal {
        Self(self.0 * other.0)
    }
}

impl MulAssign<Decimal> for Decimal {
    fn mul_assign(&mut self, rhs: Decimal) {
        self.0 *= rhs.0;
    }
}

impl SubAssign<Decimal> for Decimal {
    fn sub_assign(&mut self, other: Decimal) {
        self.0 -= other.0;
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::ToPrimitive;

    use super::{Amount, Decimal};

    #[test]
    fn test_decimal() {
        let x = Decimal::from(5);

        assert_eq!(Some(5), x.0.to_i32());
    }

    #[test]
    fn test_division() {
        let a = Amount::new(10.into(), Some(3.into()));
        let b = Amount::new(5.into(), Some(3.into()));
        let expected = Amount::new(2.into(), Some(3.into()));

        let c = a / b;

        assert_eq!(expected, c);
    }

    #[test]
    fn test_multiply_assign() {
        let a = Amount::from(10);
        let b = Amount::from(5);

        let actual = a * b;
    }
}
