use std::io::Read;

use crate::journal::Journal;

/**
 * Parser with iterators
 * 
 * The main idea here is to minimize memory allocations. 
 * The parsing is done in functions, not objects. 
 * Each parser will provide an iterator over the tokens it recognizes, i.e. Xact parser
 * will iterate over the Xact header items: date, payee, note.
 * Post parser provides an iterator over Account, Amount. Amount parser provides
 * sign, quantity, symbol, price.
 * Iterator returns None if a token is not present.
 * 
 * Tokens are then handled by lexer, which creates instances of Structs and populates
 * the collections in the Journal.
 * It also creates links among the models. This functionality is from finalize() function.
 */

pub(crate) fn parse<T: Read>(source: T) -> Journal {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::account::Account;

    #[test]
    fn test_minimal_parsing() {
        let input = r#"; Minimal transaction
        2023-04-10 Supermarket
            Expenses  20
            Assets
        "#;
                let cursor = Cursor::new(input);
        
                let journal = super::parse(cursor);
        
                assert_eq!(1, journal.xacts.len());
        
                let xact = journal.xacts.first().unwrap();
                assert_eq!("Supermarket", xact.payee);
                assert_eq!(2, xact.posts.len());
        
                // let post_1 = xact.posts.iter().nth(0).unwrap();
                let post1 = &journal.posts[xact.posts[0]];
                assert_eq!(Account::new("Expenses"), post1.account);
                assert_eq!("20", post1.amount.as_ref().unwrap().quantity.to_string());
                assert_eq!(None, post1.amount.as_ref().unwrap().commodity);
        
                // let post_2 = xact.posts.iter().nth(1).unwrap();
                let post2 = &journal.posts[xact.posts[1]];
                assert_eq!(Account::new("Assets"), post2.account);
    }
}