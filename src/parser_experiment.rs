/*!
 * A different approach to parsing.
 * Tailored to Wasm usage.
 * Break down steps into logical units.
 * Use the iterator?
 */

use std::io::{BufReader, Read};

use crate::{directives::DirectiveType, journalreader::JournalReader};

/// Read Journal, convert to directives
pub fn text_to_directives() {
    // reader: BufReader<T>
    todo!()

    // read line from the Journal
    // determine the type
    // DirectiveType
    // scan the line
    // parse into a model instance
    // read additional lines, as needed. Ie for Xact/Posts.
    // return the directive with the entity, if created
}

/// For wasm, it would be useful to leave the text reading to the caller.
/// Accept a block of text (read until an empty line) and create one directive.
/// Assume a xact for now?
pub fn text_to_directive(text: &str) {
    // let buffer
    let reader = JournalReader::new();
    // reader.read_line()
    // determine the type
    
    todo!()
    // DirectiveType::Price
}

#[cfg(test)]
mod tests {
    use super::text_to_directive;

    #[test]
    fn test_parsing_one_xact() {
        let text = r#"2023-07-25 Supermarket
    Expenses:Groceries  20 EUR
    Assets:Cash
"#;
        
        text_to_directive(text);

        todo!("complete");
    }
}