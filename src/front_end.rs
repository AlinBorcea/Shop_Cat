use tui::{
    widgets::{Paragraph, Block, BorderType, Borders, Tabs, List, ListItem},
    layout::{Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
};

//Functions to draw the pages
pub fn draw_tabs(menu_titles: &Vec<String>, page_index: usize) -> Tabs {
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

pub fn draw_home<'a>(content: &'a str) -> Paragraph<'a> {
    Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
        )
}

pub fn draw_list<'a>(item_names: &'a Vec<String>) -> List<'a> {
    let items: Vec<ListItem> = item_names.iter().map(|el| {
        ListItem::new(el.as_ref())
    }).collect();

    List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">")
}