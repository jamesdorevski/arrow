use std::{fmt::Debug, io::Write};

pub struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(header: Vec<String>) -> Self {
        Table {
            header,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn print(&self, output: &mut impl Write) {
        let mut max_widths: Vec<usize> = vec![0; self.header.len()];
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > max_widths[i] {
                    max_widths[i] = cell.len();
                }
            }
        }

        let mut header = String::new();
        for (i, cell) in self.header.iter().enumerate() {
            header.push_str(&format!("{:width$} ", cell.to_uppercase(), width = max_widths[i]));
        }
        
        match write!(output, "{}\n", header) {
            Ok(_) => {},
            Err(e) => eprintln!("Error writing to output: {}", e),
        }

        for row in &self.rows {
            let mut row_str = String::new();
            for (i, cell) in row.iter().enumerate() {
                row_str.push_str(&format!("{:width$} ", cell, width = max_widths[i]));
            }
            match write!(output, "{}\n", row_str) {
                Ok(_) => {},
                Err(e) => eprintln!("Error writing to output: {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_print() {
        let mut table = Table::new(vec!["Name".to_string(), "Age".to_string(), "City".to_string()]);
        table.add_row(vec!["John".to_string(), "25".to_string(), "New York".to_string()]);
        table.add_row(vec!["Jane".to_string(), "30".to_string(), "London".to_string()]);
        table.add_row(vec!["Mike".to_string(), "40".to_string(), "Paris".to_string()]);

        let mut output = Vec::new();
        table.print(&mut output);

        let expected_output = "NAME AGE CITY     \n\
                               John 25 New York \n\
                               Jane 30 London   \n\
                               Mike 40 Paris    \n";

        assert_eq!(String::from_utf8(output).unwrap(), expected_output);
    }
}
