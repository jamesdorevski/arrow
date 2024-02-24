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

    pub fn print(&self) {
        let mut max_widths: Vec<usize> = vec![2; self.header.len()];
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

        println!("{}", header);
        for row in &self.rows {
            let mut row_str = String::new();
            for (i, cell) in row.iter().enumerate() {
                row_str.push_str(&format!("{:width$} ", cell, width = max_widths[i]));
            }
            println!("{}", row_str);
        }
    }
}