use crate::journal::Journal;

/**
 * Reports
 */

fn balance_report(journal: Journal) {
    // iterate over accounts

    // ???
    
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{journal::Journal, xact::Xact, post::Post, amount::Amount};

    use super::balance_report;

    fn create_journal() -> Journal {
        let mut xact = Xact::new(None, "Payee", None);
        let mut post = Post::new();
        post.account = "Assets".into();
        post.amount = Amount::parse("20 EUR");
        xact.add_post(post);

        post = Post::new();
        post.account = "Expenses".into();
        xact.add_post(post);

        let mut journal = Journal::new();
        journal.add_xact(xact);

        journal
    }

    #[test]
    fn test_balance_report_one_xact() {
        let journal = create_journal();

        let actual = balance_report(journal);

        assert!(false)
    }
}