/*!
 * Implemetation of the parser that returns an iterator over the results
 */

use crate::simplemodel::SimpleXact;


/// Types of directives
#[derive(Debug)]
 pub enum DirectiveType {
    Price,
    Xact(SimpleXact)
}

#[derive(Debug)]
/// A custom iterator type
pub struct SimpleParserIter {
    // reader
    counter: u8,
}

impl SimpleParserIter {
    pub fn new() -> Self {
        SimpleParserIter { counter: 0 }
    }
}

impl Iterator for SimpleParserIter {
    type Item = DirectiveType;

    fn next(self: &mut SimpleParserIter) -> Option<Self::Item> {
        // read the content and
        // parse the next directive

        self.counter += 1;
        if self.counter > 100 {
            return None;
        }

        Some(DirectiveType::Xact(SimpleXact::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleParserIter;

    #[test]
    /// create a custom iterator of directives
    fn test_creating_custom_iterator() {
        let item = SimpleParserIter::new();

        for x in item {
            println!("item: {:?}", x);
        }
    }
}