use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event, KeyCode};

use std::sync::mpsc;
use std::thread;
use std::io;

use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Layout, Constraint, Direction, Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Paragraph, Block, BorderType, Borders, Tabs, List, ListItem, ListState},
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
    let header_titles = vec!["Home".to_owned(), "Table List".to_owned(), "Table Editor".to_owned()];
    let mut header_index = 0;

    let home_content = "\n\nPress keys F1 - F3 to select the desired page.\nFor each page follow the instructions!";
    let table_names = vec!["dogs", "cats", "owners", "drugs"];
    let mut table_state = ListState::default();
    table_state.select(Some(0));

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

            let main_tab = draw_tabs(&header_titles, header_index);
            rect.render_widget(main_tab, main_chunks[0]);

            match header_index {
                0 => {rect.render_widget(draw_home(home_content), main_chunks[1]);}
                1 => {rect.render_stateful_widget(draw_list(&table_names), main_chunks[1], &mut table_state);}
                _ => {rect.render_widget(draw_home(home_content), main_chunks[1]);}
            }

        })?;

        //Input handler
        match rx.recv()? {
            event => match event.code {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                KeyCode::F(u) => {
                    if u >= 1u8 && u <= header_titles.len() as u8 {
                        header_index = (u - 1) as usize;
                    }
                }
                KeyCode::Up => {
                    match header_index {
                        1 => {
                            let mut index = table_state.selected().unwrap();
                            if index == 0 {
                                index = table_names.len() - 1;
                            } else {
                                index -= 1;
                            }
                    
                            table_state.select(Some(index));
                        }
                        _ => {}
                    }                    
                }

                KeyCode::Down => {
                    match header_index {
                        1 => {
                            let mut index = table_state.selected().unwrap();
                            if index + 1 >= table_names.len() {
                                index = 0;
                            } else {
                                index += 1;
                            }
                    
                            table_state.select(Some(index));
                        }
                        _ => {}
                    }                    
                }
                _ => {}
            }
        }
    }

    Ok(())
}

//Functions to draw the pages
fn draw_tabs(menu_titles: &Vec<String>, page_index: usize) -> Tabs {
    let menu = menu_titles
        .iter()
        .map(|t| {
            Spans::from(vec![
                Span::styled(t, Style::default())
            ])
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

fn draw_list<'a>(item_names: &'a Vec<&str>) -> List<'a> {
    let items: Vec<ListItem> = item_names.iter().map(|el| {
        ListItem::new(el.as_ref())
    }).collect();

    List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">")
}