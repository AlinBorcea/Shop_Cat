use tui::{
    widgets::{Paragraph, Block, BorderType, Borders, Tabs, List, ListItem, Table, Cell, Row},
    layout::{Alignment, Constraint},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
};

//Functions to draw the pages

pub fn draw_paragraph<'a>(content: &'a str) -> Paragraph<'a> {
    Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
        )
}

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
    draw_paragraph(content)
}

pub fn draw_list<'a>(item_names: &'a Vec<String>) -> List<'a> {
    let items: Vec<ListItem> = item_names.iter().map(|el| {
        ListItem::new(el.as_ref())
    }).collect();

    List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC).bg(Color::White).fg(Color::Black))
    .highlight_symbol(">")
}

pub fn draw_table<'a>(name: &'a str, header: &'a Vec<String>, rows: &'a Vec<Vec<String>>) -> Table<'a> {
    let header_cells: Vec<Cell> = header.iter().map(|t| {
        Cell::from(t.as_ref())
    }).collect();
    
    let header_data = Row::new(
        header_cells
    );

    let mut this_rows: Vec<Row> = Vec::with_capacity(rows.len());
    for row in rows {
        let c: Vec<Cell> = row.iter().map(|t| {
            Cell::from(t.as_ref())
        }).collect();

        this_rows.push(Row::new(c));
    }
    
    Table::new(this_rows)
    .block(Block::default().title(name).borders(Borders::ALL))
    .header(header_data)
    .widths(&[Constraint::Length(20), Constraint::Length(20), Constraint::Length(20), Constraint::Length(20), Constraint::Length(20), Constraint::Length(20)])
    .column_spacing(4)
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Red))
}