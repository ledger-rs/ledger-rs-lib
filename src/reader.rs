//! The Journal Reader.
//! Reads directives from the given source and returns them as an iterator.

use std::io::{BufRead, BufReader, Cursor, Read};

use crate::directives::DirectiveType;

pub fn create_reader<T: Read>(source: T) -> DirectiveIter<T> {
    let iter = DirectiveIter::new(source);
    iter
}

pub fn create_str_reader<T: Read>(source: &str) -> DirectiveIter<Cursor<&str>> {
    let cursor = Cursor::new(source);
    let iter: DirectiveIter<Cursor<&str>> = DirectiveIter::new(cursor);
    // read(cursor)
    iter
}

pub struct DirectiveIter<T: Read> {
    // source: T,
    reader: BufReader<T>,
    buffer: String,
}

impl<T: Read> DirectiveIter<T> {
    pub fn new(source: T) -> Self {
        let reader = BufReader::new(source);

        Self {
            // source,
            reader,
            buffer: String::new(),
        }
    }
}

impl<T: Read> Iterator for DirectiveIter<T> {
    type Item = DirectiveType;

    fn next(self: &mut DirectiveIter<T>) -> Option<Self::Item> {
        // Read lines and recognise the directive.
        match self.reader.read_line(&mut self.buffer) {
            Err(error) => panic!("Error: {:?}", error),
            Ok(0) => {
                // end of file
                return None;
            }
            Ok(result) => {
                // TODO: Recognise directive, if any.
                // TODO: Read additional lines, if needed (like for Xact).
                // TODO: Parse and return the directive.

                // return Some(DirectiveType::Xact(Xact::default()))
                println!("Result: {:?}", result);
                return Some(DirectiveType::Comment);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::reader::create_str_reader;

    #[test]
    fn basic_test() {
        let content = "; blah blah";

        let output = create_str_reader::<Cursor<&str>>(content);

        let mut counter = 0;
        for item in output {
            println!("item: {:?}", item);
            counter += 1;
        }
        assert_eq!(1, counter);
    }

    // #[test]
    fn iterator_test() {
        let content = "blah blah";
        //let iter = DirectiveIter::new();
        let iter = create_str_reader::<Cursor<&str>>(content);

        for x in iter {
            println!("Directive: {:?}", x);
        }
    }
}
