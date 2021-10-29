use crossterm::event::KeyCode;
use tui::widgets::ListState;

//Input handlers
pub fn handle_table_list_input(code: &KeyCode, state: &mut ListState, length: usize) {
    match code {
        KeyCode::Up => {
            let mut index = state.selected().unwrap();
            if index == 0 {
                index = length - 1;
            } else {
                index -= 1;
            }

            state.select(Some(index));
        },
        KeyCode::Down => {
            let mut index = state.selected().unwrap();
            if index + 1 >= length {
                index = 0;
            } else {
                index += 1;
            }
            
            state.select(Some(index));
        },
        _ => {}
    };
}

pub fn handle_add_table_input(code: &KeyCode, buffer: &mut String, table: &mut Vec<Vec<String>>, row: &mut usize, column: &mut usize) {
    let row_count = table.len();
    let column_count = table[0].len();

    match code {
        KeyCode::Backspace => {
            buffer.pop();
        },
        KeyCode::Enter => {
            if *row < row_count && *column < column_count {
                table[*row][*column] = buffer.clone();
                buffer.clear();
            }
        },
        KeyCode::Up => {
            if *row == 0 {
                *row = row_count - 1;
            } else {
                *row -= 1;
            }
        },
        KeyCode::Down => {
            if *row == row_count - 1 {
                *row = 0;
            } else {
                *row += 1;
            }
        },
        KeyCode::Left => {
            if *column == 0 {
                *column = column_count - 1;
            } else {
                *column -= 1;
            }
        },
        KeyCode::Right => {
            if *column == column_count - 1 {
                *column = 0;
            } else {
                *column += 1;
            }
        },
        KeyCode::Char(ch) => {
            buffer.push(*ch);
        },
        _ => {}
    }
}