use crate::{journal::Journal, filters::calc_posts};

/**
 * Reports
 */

/// Balance report. Invoked with 'b' command.
/// Or accounts_report in ledger.
fn balance_report(journal: Journal) -> Vec<String> {
    // filters:
    // - amount
    // - total
    // - display amount
    // - display total
    // revalued total

    // iterate over posts

    // calc_posts::operator() is in filters.cc
    calc_posts();

    // accounts_flusher
    
    todo!()
}

fn accounts_flusher_operator() {
    // create accounts iterator
    // pass_down_accounts
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{journal::Journal, xact::Xact, post::Post, amount::Amount};

    use super::balance_report;

    fn create_journal() -> Journal {
        let mut xact = Xact::new(None, "Payee", None);
        let mut post = Post::new("Assets".into(), Amount::parse("20 EUR"));
        xact.add_post(post);

        post = Post::new("Expenses".into(), None);
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