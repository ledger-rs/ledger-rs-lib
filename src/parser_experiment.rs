/*!
 * A different approach to parsing.
 * Tailored to Wasm usage.
 * Break down steps into logical units.
 * Use the iterator?
 */

use std::io::{BufReader, Read};

use crate::iterator::DirectiveType;

/// Read Journal, convert to directives
pub fn text_to_directives() {
    // reader: BufReader<T>
    // todo

    // read line from the Journal
    // determine the type
    // DirectiveType
    // scan the line
    // parse into a model instance
    // read additional lines, as needed. Ie for Xact/Posts.
    // return the directive with the entity, if created
}