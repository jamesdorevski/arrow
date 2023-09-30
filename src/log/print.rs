use prettytable::{Table, row};

use crate::model::Log;

pub fn print_table(logs: &Vec<Log>) {
    let mut table = Table::new();

    table.add_row(row!["MESSAGE", "START", "END", "DURATION"]);

    for l in logs {
        let message = l.message.unwrap_or(String::default());
        let start_str = l.unwrap_start_str();
        let end_str = l.unwrap_end_str();

        table.add_row(row![message, start_str, end_str, l.duration]);
    }

    table.printstd();
}
