use std::cell::Cell;

use crate::{journal::Journal, xact::Xact};

/**
 * Parsing context
 * 
 * Provides context data and temporary storage during parsing operation.
 */

pub(crate) struct ParsingContext {
    pub journal: Journal,

    /// Transaction being parsed currently. If exist, we are in the process of parsing posts.
    pub xact: Option<Xact>,

}

impl ParsingContext {
    pub fn new() -> Self {
        Self {
            xact: None,
            // commodity_pool: CommodityPool::new(),
            journal: Journal::new(),
        }
    }
}
