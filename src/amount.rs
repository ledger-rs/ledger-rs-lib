/*!
 * Amount and the Decimal numeric type
 */

use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, MulAssign, SubAssign},
};

use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

use crate::commodity::Commodity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Amount {
    pub quantity: Quantity,
    pub(crate) commodity: *const Commodity,
}

impl Amount {
    pub fn new(quantity: Quantity, commodity: Option<*const Commodity>) -> Self {
        Self {
            quantity,
            commodity: if commodity.is_some() {
                commodity.unwrap() as *const Commodity
            } else {
                std::ptr::null()
            },
        }
    }

    /// Returns an absolute (positive) Amount.
    pub fn abs(&self) -> Amount {
        let mut result = self.clone();
        result.quantity.set_sign_positive();
        result
    }

    pub fn copy_from(other: &Amount) -> Self {
        Self {
            quantity: other.quantity,
            commodity: other.commodity,
        }
    }

    pub fn null() -> Self {
        Self {
            quantity: 0.into(),
            commodity: std::ptr::null(),
        }
    }

    pub fn add(&mut self, other: &Amount) {
        if self.commodity != other.commodity {
            log::error!("different commodities");
            panic!("don't know yet how to handle this")
        }
        if other.quantity.is_zero() {
            // nothing to do
            return;
        }

        self.quantity += other.quantity;
    }

    pub fn get_commodity(&self) -> Option<&Commodity> {
        if self.commodity.is_null() {
            None
        } else {
            unsafe { 
                Some(&*self.commodity)
            }
        }
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

        unsafe { Amount::new(new_quantity, Some(&*self.commodity)) }
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
            return self.commodity.is_null();
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        self.quantity.is_zero()
    }

    pub fn remove_commodity(&mut self) {
        self.commodity = std::ptr::null();
    }
}

impl Add<Amount> for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Self::Output {
        if self.commodity != rhs.commodity {
            panic!("don't know yet how to handle this")
        }

        let sum = self.quantity + rhs.quantity;

        unsafe { Amount::new(sum, Some(&*self.commodity)) }
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

impl Div for Amount {
    type Output = Amount;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result = Amount::new(0.into(), None);

        if self.commodity.is_null() {
            result.commodity = rhs.commodity;
        } else {
            result.commodity = self.commodity
        }

        result.quantity = self.quantity / rhs.quantity;

        result
    }
}

impl Mul<Amount> for Amount {
    type Output = Amount;

    fn mul(self, other: Amount) -> Amount {
        let quantity = self.quantity * other.quantity;

        let commodity = if self.commodity.is_null() {
            other.commodity
        } else {
            self.commodity
        };

        unsafe { Amount::new(quantity, Some(&*commodity)) }
    }
}

impl From<i32> for Amount {
    fn from(value: i32) -> Self {
        Amount::new(Quantity::from(value), None)
    }
}

impl SubAssign<Amount> for Amount {
    fn sub_assign(&mut self, other: Amount) {
        if self.commodity != other.commodity {
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
        if self.commodity.is_null() && !rhs.commodity.is_null() {
            self.commodity = rhs.commodity;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quantity(rust_decimal::Decimal);

impl Quantity {
    pub const ZERO: Quantity = Quantity(rust_decimal::Decimal::ZERO);
    pub const ONE: Quantity = Quantity(rust_decimal::Decimal::ONE);

    pub fn from_str(str: &str) -> Option<Self> {
        let parsed = rust_decimal::Decimal::from_str_exact(str);
        if parsed.is_err() {
            return None;
        }

        Some(Self(parsed.unwrap()))
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

impl From<i32> for Quantity {
    fn from(value: i32) -> Self {
        Quantity(rust_decimal::Decimal::from(value))
    }
}

impl From<f32> for Quantity {
    fn from(value: f32) -> Self {
        Quantity(rust_decimal::Decimal::from_f32(value).unwrap())
    }
}

/// Creates a Decimal value from a string. Panics if invalid.
impl From<&str> for Quantity {
    fn from(value: &str) -> Self {
        Self(rust_decimal::Decimal::from_str_exact(value).unwrap())
        // Decimal::from_str(value).unwrap()
    }
}

impl Into<i32> for Quantity {
    fn into(self) -> i32 {
        self.0.to_i32().unwrap()
    }
}

impl Add<Quantity> for Quantity {
    type Output = Quantity;

    fn add(self, other: Quantity) -> Quantity {
        Quantity(self.0 + other.0)
    }
}

impl AddAssign<Quantity> for Quantity {
    fn add_assign(&mut self, other: Quantity) {
        self.0 += other.0;
    }
}

impl Div<Quantity> for Quantity {
    type Output = Quantity;

    fn div(self, other: Quantity) -> Quantity {
        Self(self.0.div(other.0))
    }
}

impl Mul<Quantity> for Quantity {
    type Output = Quantity;

    fn mul(self, other: Quantity) -> Quantity {
        Self(self.0 * other.0)
    }
}

impl MulAssign<Quantity> for Quantity {
    fn mul_assign(&mut self, rhs: Quantity) {
        self.0 *= rhs.0;
    }
}

impl SubAssign<Quantity> for Quantity {
    fn sub_assign(&mut self, other: Quantity) {
        self.0 -= other.0;
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::ToPrimitive;

    use crate::commodity::Commodity;

    use super::{Amount, Quantity};

    #[test]
    fn test_decimal() {
        let x = Quantity::from(5);

        assert_eq!(Some(5), x.0.to_i32());
    }

    #[test]
    fn test_division() {
        let currency = Commodity::new("EUR");
        let a = Amount::new(10.into(), Some(&currency));
        let b = Amount::new(5.into(), Some(&currency));
        let expected = Amount::new(2.into(), Some(&currency));

        let c = a / b;

        assert_eq!(expected, c);
    }

    #[test]
    fn test_multiply_assign() {
        let a = Amount::from(10);
        let b = Amount::from(5);

        let actual = a * b;

        assert_eq!(Amount::new(Quantity::from_str("50").unwrap(), None), actual);
    }
}
