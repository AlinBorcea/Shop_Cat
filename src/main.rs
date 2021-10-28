use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};

use std::sync::mpsc;
use std::thread;
use std::io;

use rusqlite::{Connection, Result};

use tui::{
    widgets::{ListState, Paragraph},
    layout::{Layout, Constraint, Direction},
    backend::CrosstermBackend,
    Terminal,
};

mod front_end;

use front_end::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    
    //Sender Receiver channel
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            if let Event::Key(key) = event::read().expect("can read") {
                tx.send(key).expect("can send key");
            }
        }
    });

    //Terminal initialization
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    //App data

    //header
    let menu_titles = vec!["Home".to_owned(), "Table List".to_owned(), "Add table".to_owned(), "Table View".to_owned()];
    let mut menu_index = 0;

    //home
    let home_content = "\n\nPress keys F1 - F3 to select the desired page.\nPress arrow keys to go down and up.\nFor each page follow the instructions!";
    
    //table list
    let mut table_names: Vec<String> = Vec::with_capacity(10);
    let mut table_list_state = ListState::default();
    let mut selected_table_name_index: isize = if table_names.len() > 0 {0} else {-1};
    table_list_state.select(Some(0));
    init_table_names(&mut table_names)?;

    //Add Table
    let def_table_header = vec![String::from("Name"), String::from("Data Type"), String::from("Default Value"), String::from("Minimum Length"), String::from("Maximum Length"), String::from("Precission")];
    let def_table_rows: Vec<Vec<String>> = vec![
        vec![String::from("name"), String::from("age"), String::from("age"), String::from("age"), String::from("age"), String::from("age")],
        vec![String::from("name"), String::from("age"), String::from("age"), String::from("age"), String::from("age"), String::from("age")],
    ];

    loop {
        //Tui drawing
        terminal.draw(|rect| {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                      Constraint::Length(3),
                      Constraint::Min(10),
                    ]
                ).split(rect.size());

            rect.render_widget(draw_tabs(&menu_titles, menu_index), main_chunks[0]);

            match menu_index {
                0 => { rect.render_widget(draw_home(home_content), main_chunks[1]); }
                1 => {
                    if table_names.len() > 0 {
                        rect.render_stateful_widget(draw_list(&table_names), main_chunks[1], &mut table_list_state);
                    }
                }
                2 => {
                    let add_table_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints([
                            Constraint::Min(3),
                            Constraint::Length(3),
                    ]).split(main_chunks[1]);

                    rect.render_widget(draw_table("Define Table", &def_table_header, &def_table_rows), add_table_chunks[0]);
                    rect.render_widget(Paragraph::new("Goes here"), add_table_chunks[1]);
                }
                _ => {}
            }

        })?;

        //Input handler
        let event = rx.recv()?;
        match event.code {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::F(u) => {
                    if u >= 1u8 && u <= menu_titles.len() as u8 {
                        menu_index = (u - 1) as usize;
                    }
                }
                _ => match menu_index {
                        1 => handle_table_list_input(&event.code, &mut table_list_state, table_names.len(), &mut selected_table_name_index),
                        _ => {}
                }
        }
    }

    Ok(())
}

//Input handlers
fn handle_table_list_input(code: &KeyCode, state: &mut ListState, length: usize, selected: &mut isize) {
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

fn init_table_names(table_names: &mut Vec<String>) -> Result<()> {
    let conn = Connection::open("_tables")?;

    //conn.execute("CREATE TABLE IF NOT EXISTS _tables
    //(
    //    name TEXT UNIQUE
    //)", [])?;

    //conn.execute("INSERT INTO _tables (name) VALUES (?1)", params!["caine"])?;
    //conn.execute("INSERT INTO _tables (name) VALUES (?1)", params!["tigru"])?;
    //conn.execute("INSERT INTO _tables (name) VALUES (?1)", params!["leu"])?;

    let mut stmt = conn.prepare("SELECT * FROM _tables")?;
    let mut rows = stmt.query([])?;
    
    while let Some(row) = rows.next()? {
        table_names.push(row.get::<usize, String>(0).unwrap());
    }

    Ok(())
}