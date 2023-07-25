/*!
 * Implemetation of the parser that returns an iterator over the results
 */


/// Types of directives
#[derive(Debug)]
 pub enum DirectiveType {
    Price,
    Xact
}

#[derive(Debug)]
/// A custom iterator type
pub struct ParserIter {
    // reader
    counter: u8,
}

impl ParserIter {
    pub fn new() -> Self {
        ParserIter { counter: 0 }
    }
}

impl Iterator for ParserIter {
    type Item = DirectiveType;

    fn next(self: &mut ParserIter) -> Option<Self::Item> {
        // parse the next directive
        //todo!("incomplete")
        //Some(self.0)
        self.counter += 1;
        if self.counter > 100 {
            return None;
        }
        
        Some(DirectiveType::Xact)
    }
}

#[cfg(test)]
mod tests {
    use super::ParserIter;

    #[test]
    /// create a custom iterator of directives
    fn test_creating_custom_iterator() {
        // let vector: Vec<String> = vec![];
        // let a = vector.iter();
        let item = ParserIter::new();

        for x in item {
            println!("item: {:?}", x);
        }
    }
}