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
    // pub xact: Option<Xact>,
    /// Indicates if we are parsing a transaction. Contains the index of the current transaction.
    pub current_xact_index: Option<usize>,
}

impl ParsingContext {
    pub fn new() -> Self {
        Self {
            // xact: None,
            // commodity_pool: CommodityPool::new(),
            journal: Journal::new(),
            current_xact_index: None,
        }
    }
}
