/**
 * utils.h
 */

/// Find the next non-whitespace character location.
/// Returns the index of the first non-ws character.
pub fn peek_next_nonws(line: &str) -> usize {
    for (i, c) in line.char_indices() {
        if c == ' ' {
            continue;
        } else {
            // got to the first non-ws
            return i;
        }
    }
    return 0;
}
