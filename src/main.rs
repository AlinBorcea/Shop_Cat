use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};

use std::sync::mpsc;
use std::thread;
use std::io;

use rusqlite::{Connection, Result};

use tui::{
    widgets::{ListState},
    layout::{Layout, Constraint, Direction},
    backend::CrosstermBackend,
    Terminal,
};

mod input_handler;
mod front_end;

use input_handler::*;
use front_end::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HOME_INDEX: usize = 0;
    const TABLE_LIST_INDEX: usize = 1;
    const ADD_TABLE_INDEX: usize = 2;
    //const TABLE_VIEW_INDEX: usize = 3;

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
    table_list_state.select(Some(0));
    init_table_names(&mut table_names)?;

    //Add Table
    let default_table_header = vec![String::from("Name"), String::from("Data Type"), String::from("Default Value"), String::from("Minimum Length"), String::from("Maximum Length"), String::from("Precission")];
    let mut default_table_rows = vec![vec![String::with_capacity(32); 6]; 10];
    let mut input_buffer = String::with_capacity(32);
    let mut current_row = 0;
    let mut current_column = 0;

    //Layouts
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(10)]);

    let add_table_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Min(3), Constraint::Length(3)]);

    loop {
        //Tui drawing
        terminal.draw(|rect| {
            let main_chunks = main_layout.split(rect.size());

            rect.render_widget(draw_tabs(&menu_titles, menu_index), main_chunks[0]);

            match menu_index {
                HOME_INDEX => {
                    rect.render_widget(draw_home(home_content), main_chunks[1]); 
                }
                TABLE_LIST_INDEX => {
                    if table_names.len() > 0 {
                        rect.render_stateful_widget(draw_list(&table_names), main_chunks[1], &mut table_list_state);
                    }
                }
                ADD_TABLE_INDEX => {
                    let add_table_chunks = add_table_layout.split(main_chunks[1]);

                    rect.render_widget(draw_table("Define Table", &default_table_header, &default_table_rows), add_table_chunks[0]);
                    rect.render_widget(draw_paragraph(input_buffer.as_ref()), add_table_chunks[1]);
                }
                _ => {}
            }

        })?;

        //Main input handler
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
                        TABLE_LIST_INDEX => handle_table_list_input(&event.code, &mut table_list_state, table_names.len()),
                        ADD_TABLE_INDEX => handle_add_table_input(&event.code, &mut input_buffer, &mut default_table_rows, &mut current_row, &mut current_column),
                        _ => {}
                }
        }
    }

    Ok(())
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