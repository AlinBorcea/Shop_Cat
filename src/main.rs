use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};

use std::sync::mpsc;
use std::thread;
use std::io;

use rusqlite::{Connection, Result};

use tui::{
    widgets::{Paragraph, Block, BorderType, Borders, Tabs, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction, Alignment},
    style::{Color, Style, Modifier},
    backend::CrosstermBackend,
    text::{Span, Spans},
    Terminal,
};

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
    let menu_titles = vec!["Home".to_owned(), "Table List".to_owned(), "Table Editor".to_owned()];
    let mut menu_index = 0;

    //home
    let home_content = "\n\nPress keys F1 - F3 to select the desired page.\nPress arrow keys to go down and up.\nFor each page follow the instructions!";
    
    //table view
    let mut table_names: Vec<String> = Vec::with_capacity(10);
    let mut table_state = ListState::default();
    table_state.select(Some(0));

    init_table_names(&mut table_names)?;

    loop {
        //Tui drawing
        terminal.draw(|rect| {
            let size = rect.size();
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                      Constraint::Length(3),
                      Constraint::Min(10),
                    ]
                ).split(size);

            rect.render_widget(draw_tabs(&menu_titles, menu_index), main_chunks[0]);

            match menu_index {
                0 => {rect.render_widget(draw_home(home_content), main_chunks[1]);}
                1 => {
                    if table_names.len() > 0 {
                        rect.render_stateful_widget(draw_list(&table_names), main_chunks[1], &mut table_state);
                    }
                }
                _ => {rect.render_widget(draw_home(home_content), main_chunks[1]);}
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
                        1 => handle_table_list_input(&event.code, &mut table_state, table_names.len()),
                        _ => {}
                }
        }
    }

    Ok(())
}

//Functions to draw the pages
fn draw_tabs(menu_titles: &Vec<String>, page_index: usize) -> Tabs {
    let menu = menu_titles.iter().map(|t| {
            Spans::from(vec![Span::styled(t, Style::default())])
    }).collect();

    Tabs::new(menu)
        .select(page_index)
        .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED).fg(Color::Red))
        .divider(Span::raw("|"))
        .block(Block::default()
            .title("Shop Cat")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded))
}

fn draw_home<'a>(content: &'a str) -> Paragraph<'a> {
    Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
        )
}

fn draw_list<'a>(item_names: &'a Vec<String>) -> List<'a> {
    let items: Vec<ListItem> = item_names.iter().map(|el| {
        ListItem::new(el.as_ref())
    }).collect();

    List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">")
}

//Input handlers
fn handle_table_list_input(code: &KeyCode, state: &mut ListState, length: usize) {
    match code {
        KeyCode::Up => {
            let mut index = state.selected().unwrap();
            if index == 0 {
                index = length - 1;
            } else {
                index -= 1;
            }
                    
            state.select(Some(index));
        }
        KeyCode::Down => {
            let mut index = state.selected().unwrap();
            if index + 1 >= length {
                index = 0;
            } else {
                index += 1;
            }
            
            state.select(Some(index));
        }
        _ => {}
    };
}

fn init_table_names(table_names: &mut Vec<String>) -> Result<()> {
    let conn = Connection::open("_tables")?;
    let mut stmt = conn.prepare("SELECT * FROM _tables")?;
    let mut rows = stmt.query([])?;
    
    while let Some(row) = rows.next()? {
        table_names.push(row.get::<usize, String>(0).unwrap());
    }

    Ok(())
}