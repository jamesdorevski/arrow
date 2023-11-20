use prettytable::{Table, row};

use crate::model::Log;

pub fn print_table(logs: &Vec<Log>) {
    let mut table = Table::new();

    table.add_row(row!["ID", "MESSAGE", "START", "END", "DURATION (s)"]);

    for l in logs {
        let msg = l.message.clone().unwrap_or(String::default());
        let start_str = l.unwrap_start_str();
        let end_str = l.unwrap_end_str();

        table.add_row(row![l.id, msg, start_str, end_str, l.duration]);
    }

    table.printstd();
}
