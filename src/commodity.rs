use crate::utils;

pub(crate) struct Commodity {
    symbol: String,
    // graph_index
    // precision
    // name
    // note
    // smaller
    // larger
    // value_expr
}

impl Commodity {
    pub fn new(symbol: String) -> Self {
        Self { symbol }
    }
}

/// Parse symbol from the input string.
/// Original code in
/// void commodity_t::parse_symbol(std::istream& in, string& symbol)
pub fn parse_symbol(input: &str) -> &str {
    // skip ws
    let c = utils::peek_next_nonws(input);

    // symbols in quotes
    if input.chars().skip(c).next() == Some('\"') {
        todo!("read everything until the closing quote")
    } else {
        // todo invalid characters? Does Rust have the same limitation?
        
        //let buf = &input[c..];
        // is_reserved_token

    }

    &input[c..]
}
