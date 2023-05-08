/**
 * Parsing context
 * 
 * Provides context data and temporary storage during parsing operation.
 */

 use crate::{journal::Journal, xact::Xact, post::Post};

pub(crate) struct ParsingContext {
    pub journal: Journal,

    /// Transaction being parsed currently. If exist, we are in the process of parsing posts.
    pub xact: Option<Xact>,
    pub posts: Vec<Post>,
}

impl ParsingContext {
    pub fn new() -> Self {
        Self {
            // commodity_pool: CommodityPool::new(),
            journal: Journal::new(),
            // cache:
            xact: None,
            posts: vec![]
        }
    }
}
