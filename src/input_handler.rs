use crossterm::event::KeyCode;
use tui::widgets::ListState;

//Input handlers
pub fn handle_table_list_input(code: &KeyCode, state: &mut ListState, length: usize, selected: &mut isize) {
    match code {
        KeyCode::Enter => {
            *selected = state.selected().unwrap() as isize;
        },
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