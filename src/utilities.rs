/*!
 * Handy utility functions
 */

use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

use crate::parser::ISO_DATE_FORMAT;

/// Create DateTime from date string only.
pub fn create_date(iso_str: &str) -> Result<NaiveDateTime, anyhow::Error> {
    Ok(NaiveDateTime::new(NaiveDate::parse_from_str(iso_str, ISO_DATE_FORMAT)?, NaiveTime::MIN))
}
