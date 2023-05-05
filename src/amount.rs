use crate::{context::ParsingContext, commodity::{Commodity, self}, utils};

/**
 * Amount
 */

 pub struct Amount {
    precision: u16,
    quantity: i64,
    // commodity: Option<&Commodity>,
    commodity: Option<Commodity>,
}

impl Amount {
    pub fn new() -> Self {
        Self {
            precision: 0,
            quantity: 0,
            commodity: None,
        }
    }

    pub(crate) fn from(quantity: i64, precision: u16, commodity: Option<Commodity>) -> Self {
        Self { precision, quantity, commodity }
    }

    /// Parse amount
    /// amount.cc
    /// bool amount_t::parse(std::istream& in, const parse_flags_t& flags)
    /// The possible syntax for an amount is:
    ///   [-]NUM[ ]SYM [@ AMOUNT]
    ///   SYM[ ][-]NUM [@ AMOUNT]
    pub(crate) fn parse(context: &mut ParsingContext, input: &str) -> Amount {
        let symbol: &str;
        let quant: &str;
        // details
        let mut negative = false;

        // cursor for chars.
        let mut c = utils::peek_next_nonws(input);
        let mut next_char = input.chars().skip(c).next();

        if next_char == Some('-') {
            // TODO: complete the negative number parsing.
            negative = true;
            c = utils::peek_next_nonws(input);
            next_char = input.chars().skip(c).next();
        }

        if next_char.unwrap().is_digit(10) {
            let offset: usize;
            (quant, offset) = parse_quantity(input);
            // move the cursor
            c += offset;

            // COMMODITY_STYLE_SEPARATED

            symbol = commodity::parse_symbol(&input[c..]);
        } else {
            todo!("commodity-first format, i.e. $25")
        }

        if quant.is_empty() {
            panic!("No quantity specified for amount")
        }

        // Create the commodity if has not already been seen, and update the
        // precision if something greater was used for the quantity.

        let mut amount = Amount::new();

        if symbol.is_empty() {
            // amount.commodity = None;
        } else {
            // TODO: use a reference
            // let commodity = context.commodity_pool.find(symbol);
            // TODO: create if not found.
            // amount.commodity = Cell::new(commodity);
            // For now, just use the symbol.
            amount.commodity = Some(Commodity::new(symbol.to_string()));
        }

        // precision
        let mut decimal_offset: u16 = 0;
        // iterate through characters backwards, searching for the decimal separator.
        for c in quant.chars().rev() {
            if c == '.' {
                // TODO: check for multiple decimal separators
                // todo: decimal comma style
                todo!("process")
            } else if c == ',' {
                todo!("process")
            } else {
                decimal_offset += 1;
            }
        }
        // todo: assign precision to commodity
        // if amount.precision > commodity.precision

        amount.quantity = i64::from_str_radix(quant, 10).expect("parsed quantity");
        // TODO: amount.precision

        // TODO: negative number

        amount
    }
}

/// Identifies the quantity in the input string.
/// Returns the str of the quantity value and
/// the last position index (for cursor control).
/// Parameters:
/// input
fn parse_quantity(input: &str) -> (&str, usize) {
    let mut start: usize = 0;
    let mut end: usize = 0;

    if input.chars().next() == Some('-') {
        start += 1;
    }

    // read characters so long as they are numeric.
    for (i, c) in input.char_indices().skip(start) {
        if c.is_digit(10) || c == '.' || c == ',' {
            continue;
        } else {
            return (&input[start..i], i);
        }
    }
    return ("", 0);
}
