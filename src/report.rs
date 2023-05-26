use rust_decimal::Decimal;

use crate::{account::Account, journal::Journal, amount::Amount, balance::Balance};

/**
 * Reports
 */

/// Accounts report. Command: `accounts`.
///
/// void report_t::posts_report(post_handler_ptr handler)
/// in output.cc
/// report_accounts
pub fn report_accounts(journal: &Journal) -> impl Iterator<Item = String> + '_ {
    journal
        .accounts
        .iter()
        .map(|account| account.name.to_string())
}

fn report_commodities() {
    todo!()
}

fn report_payees() {
    todo!()
}

/// Balance report. Invoked with 'b' command.
/// Or accounts_report in ledger.
pub fn balance_report(journal: Journal) -> Vec<String> {
    let balances = get_account_balances(journal);

    // Format output
    format_balance_report(balances)
}

/// Calculates account balances.
/// returns (account_name, balance)
/// 
fn get_account_balances(journal: Journal) -> Vec<(String, Balance)> {
    let mut balances = vec![];

    // calculate balances
    for (i, acc) in journal.accounts.iter().enumerate() {
        // TODO: separate balance per currency

        let filtered_posts = journal
            .posts
            .iter()
            .filter(|post| post.account_index == i);
            // .map(|post| post.amount)
            // .sum();
        let mut balance: Balance = Balance::new();
        for post in filtered_posts {
            balance.add(&post.amount.as_ref().unwrap());
        }

        balances.push((acc.name.to_owned(), balance));
    }
    balances
}

fn format_balance_report(mut balances: Vec<(String, Balance)>) -> Vec<String> {
    // sort accounts
    balances.sort_by(|(acc1, bal1), (acc2, bal2)| acc1.cmp(&acc2));

    let mut output = vec![];
    for (account, balance) in balances {
        let mut bal_text: String = String::new();
        for (cur_index, amount) in &balance.amounts {
            // 
            bal_text += format!("{:?} {}", amount, cur_index).as_str();
        }
        let line = format!("Account {} has balance {:?}", account, balance);
        output.push(line);
    }
    output
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{journal::Journal, parser};
    use super::balance_report;

    fn create_journal() -> Journal {
        let src = r#";
2023-05-05 Payee
    Expenses  25 EUR
    Assets

"#;
        let source = Cursor::new(src);
        let mut journal = Journal::new();
        parser::read_into_journal(source, &mut journal);
        journal
    }

    #[test]
    fn test_balance_report_one_xact() {
        let journal = create_journal();

        let actual = balance_report(journal);

        assert!(!actual.is_empty());
        assert_eq!("Account Assets has balance -25 EUR", actual[0]);
        assert_eq!("Account Expenses has balance 25 EUR", actual[1]);
    }

    #[test]
    fn test_with_two_commodities() {
        let src = r#";
2023-05-05 Payee
    Expenses  25 EUR
    Assets

2023-05-05 Payee 2
    Expenses  25 BAM
    Assets
"#;
        let source = Cursor::new(src);
        let mut journal = Journal::new();
        parser::read_into_journal(source, &mut journal);

        // Act
        let actual = balance_report(journal);

        // Assert
        assert!(!actual.is_empty());
    }
}
