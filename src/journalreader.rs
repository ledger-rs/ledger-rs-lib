/*!
 * Journal Reader reads lines from the journal string and keeps track of the 
 * line number.
 */

 /// Reads the input text line by line and keeps track of the line number.
struct JournalReader {}

impl JournalReader {
    pub fn new() -> Self {
        JournalReader { }
    }
}

#[cfg(test)]
mod tests {
    use super::JournalReader;

    #[test]
    fn test_instantiation() {
        let x = JournalReader::new();

        // if no exceptions
        assert!(true);
    }
}