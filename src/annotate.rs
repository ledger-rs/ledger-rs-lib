/*!
 * Types for annotating commodities
 *
 * annotate.h
 *
 */

use chrono::NaiveDate;

use crate::amount::Amount;

pub(crate) struct Annotation {
    price: Option<Amount>,
    date: Option<NaiveDate>,
    tag: Option<String>,
    // value_expr
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
