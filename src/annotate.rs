/*!
 * Types for annotating commodities
 *
 * annotate.h
 *
 */

use chrono::NaiveDate;

use crate::amount::Amount;

pub struct Annotation {
    pub price: Option<Amount>,
    pub date: Option<NaiveDate>,
    pub tag: Option<String>,
    // pub value_expr:
}

impl Annotation {
    pub fn new(price: Option<Amount>, date: Option<NaiveDate>) -> Self {
        // todo: add support for tags
        Self {
            price,
            date,
            tag: None,
        }
    }
}

#[cfg(test)]
mod tests {
    
}
