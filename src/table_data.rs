use crossterm::event::KeyCode;

pub struct TableData {
    name: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    row: usize,
    column: usize,
    buffer: String,
}

impl TableData {
    pub fn from(name: &String, header: &Vec<String>) -> Self {
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(10);
        let mut first: Vec<String> = Vec::with_capacity(header.len());
        let mut i = 0;
        
        while i < header.len() {
            first.push(String::with_capacity(32));
            i += 1;
        }
        rows.push(first);
        
        TableData {
            name: name.clone(),
            header: header.clone(),
            rows: rows,
            row: 0,
            column: 0,
            buffer: String::with_capacity(32),
        }
    }

    pub fn name_ref(&self) -> &str {
        self.name.as_ref()
    }

    pub fn header(&self) -> &Vec<String> {
        &self.header
    }

    pub fn rows(&self) -> &Vec<Vec<String>> {
        &self.rows
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn buffer_ref(&self) -> &str {
        self.buffer.as_ref()
    }

    pub fn handle_input(&mut self, code: &KeyCode) {
        let row_count = self.rows.len();
        let column_count = self.rows[0].len();
    
        match code {
            KeyCode::Backspace => {
                self.buffer.pop();
            },
            KeyCode::Enter => {
                if self.row < row_count && self.column < column_count {
                    self.rows[self.row][self.column] = self.buffer.clone();
                    self.buffer.clear();
                }
            },
            KeyCode::Up => {
                if self.row == 0 {
                    self.row = row_count - 1;
                } else {
                    self.row -= 1;
                }
            },
            KeyCode::Down => {
                if self.row == row_count - 1 {
                    self.row = 0;
                } else {
                    self.row += 1;
                }
            },
            KeyCode::Left => {
                if self.column == 0 {
                    self.column = column_count - 1;
                } else {
                    self.column -= 1;
                }
            },
            KeyCode::Right => {
                if self.column == column_count - 1 {
                    self.column = 0;
                } else {
                    self.column += 1;
                }
            },
            KeyCode::Char('?') => {
                self.buffer.clear();
                if Self::validate_rows(&self.rows) {
                    self.buffer.push_str("Yes");
                } else {
                    self.buffer.push_str("No");
                }
            },
            KeyCode::Char(ch) => {
                self.buffer.push(*ch);
            },
            _ => {}
        }
    }
    
    fn validate_rows(rows: &Vec<Vec<String>>) -> bool {
        if rows.len() < 1 {
            return false;
        }

        for row in rows.iter() {
            if row.len() < 2 {
                return false;
            }

            let cell_type = CellType::from(row[1].as_ref());
            if cell_type == CellType::Empty {
                return false;
            }

            if !cell_type.content_is_good(row[0].as_ref()) {
                return false;
            }
        }
    
        true
    }

}

#[derive(PartialEq, Eq)]
enum CellType {
    Empty,
    Text,
    Integer,
    Real,
}

impl CellType {
    pub fn from(cell_content: &str) -> CellType {
        match cell_content {
            "Text" => CellType::Text,
            "Integer" => CellType::Integer,
            "Real" => CellType::Real,
            _ => CellType::Empty
        }
    }

    pub fn content_is_good(&self, cell_content: &str) -> bool {
        match self {
            CellType::Text => !cell_content.is_empty(),
            CellType::Integer => match cell_content.parse::<isize>() { Ok(_) => true, _ => false},
            CellType::Real => match cell_content.parse::<f64>() { Ok(_) => true, _ => false },
            _ => false
        }
    }
}