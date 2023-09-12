use prettytable::{Table, row};
use chrono::Duration;

use crate::model::Log;

pub fn print_table(logs: &Vec<Log>) {
    let mut table = Table::new();

    table.add_row(row!["MESSAGE", "START", "END", "DURATION"]);

    for l in logs {
        table.add_row(row![l.message, l.start, l.end, l.duration]);
    }

    table.printstd();
}
