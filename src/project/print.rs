// use prettytable::{Table, row};
// use chrono::Duration;

// use crate::model::Project;

// pub fn print_table(projects: &Vec<Project>) {
//     let mut table = Table::new();
   
//     table.add_row(row!["ID", "NAME", "CREATED", "UPDATED", "TOTAL DURATION (s)"]);

//     for p in projects {
//         let total_duration = p.total_duration.unwrap_or_else(|| Duration::seconds(0));
//         table.add_row(row![p.id, p.name, p.created, p.updated, total_duration]);
//     }
    
//     table.printstd();
// }
