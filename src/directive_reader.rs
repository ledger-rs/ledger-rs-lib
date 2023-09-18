//! Directive Reader
//! Reads directives from the given source.

use std::io::{Read, Cursor, BufReader};

use crate::directives::DirectiveType;

pub fn read<T: Read>(source: T) -> DirectiveIter<T> {
    let iter = DirectiveIter::new(source);
    iter
}

pub fn read_str<T: Read>(source: &str) -> DirectiveIter<Cursor<&str>> {
    let cursor = Cursor::new(source);
    let iter: DirectiveIter<Cursor<&str>> = DirectiveIter::new(cursor);
    // read(cursor)
    iter
}

pub struct DirectiveIter<T: Read> {
    // source: T,
    reader: BufReader<T>
}

impl<T: Read> DirectiveIter<T> {
    pub fn new(source: T) -> Self {
        let reader = BufReader::new(source);

        Self {
            // source,
            reader
        }
    }
}

impl<T: Read> Iterator for DirectiveIter<T> {
    type Item = DirectiveType;

    fn next(self: &mut DirectiveIter<T>) -> Option<Self::Item> {
        // Read lines and recognise the directive.
        // TODO: Read line.
        // TODO: Recognise directive, if any.
        // TODO: Read additional lines, if needed (like for Xact).
        // TODO: Parse and return the directive.

        // return Some(DirectiveType::Xact(Xact::default()))

        // TODO: "Return None when complete";
        return None
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::directive_reader::read_str;

    use super::DirectiveIter;

    fn basic_test() {
        let content = "blah blah";
        // let reader = DirectiveIter::new();

        let output = read_str::<Cursor<&str>>(content);

        todo!("incomplete");
    }

    // #[test]
    fn iterator_test() {
        let content = "blah blah";
        //let iter = DirectiveIter::new();
        let iter = read_str::<Cursor<&str>>(content);

        for x in iter {
            println!("Directive: {:?}", x);
        }
    }
}