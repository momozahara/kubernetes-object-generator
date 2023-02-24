use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::{App, InputMode, ServiceType};

pub fn main<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(f.size());
    let chunksh = Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Kubernetes Object Generator");

        let mut menu_lists: Vec<String> = Vec::new();
        menu_lists.push(format!("name: \"{}\"", app.kube.name));
        menu_lists.push(format!("label: \"{}\"", app.kube.label));
        menu_lists.push(format!("container name: \"{}\"", app.kube.cname));
        menu_lists.push(format!("image: \"{}\"", app.kube.image));
        menu_lists.push(format!("secret: \"{}\"", app.kube.secret));
        menu_lists.push(format!(
            "service type: {}",
            match app.kube.stype {
                ServiceType::NodePort => String::from("NodePort"),
                ServiceType::LoadBalancer => String::from("LoadBalancer"),
            }
        ));
        menu_lists.push(String::from("generate"));

        let items: Vec<ListItem> = menu_lists
            .iter()
            .map(|item| {
                let span = Span::from(item.to_owned());
                ListItem::new(span).style(Style::default())
            })
            .collect();

        let items = List::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(match app.input_mode {
                InputMode::Normal => Modifier::REVERSED,
                InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
                InputMode::Select => Modifier::REVERSED | Modifier::UNDERLINED,
            }));

        f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);
    }

    {
        let block = Block::default().borders(Borders::ALL).title("Help");

        let menu_lists = match app.input_mode {
            InputMode::Normal => vec![
                "Up/K Down/J - Navigate",
                "Enter/E - Select/Edit",
                "Esc/Q - Return",
            ],
            InputMode::Insert => vec!["Backspace - Delete", "Enter/Esc - Return"],
            InputMode::Select => vec!["Enter/Esc - Return", "Up/K Down/J - Navigate"],
        };

        let items: Vec<ListItem> = menu_lists
            .iter()
            .map(|item| {
                let span = Span::from(item.to_owned());
                ListItem::new(span).style(Style::default())
            })
            .collect();

        let items = List::new(items).block(block);

        f.render_widget(items, chunksh[0]);
    }

    {
        let block = Block::default().borders(Borders::ALL).title("Example");

        let menu_lists = vec![
            "*name: \"hello-world\"",
            "*label: \"hello-app\"",
            "*container name: \"hello-world\"",
            "*image: \"docker.io/momozahara/rust-actix-hello\"",
            "secret: \"mysecret\" | \"\"",
            "*service type: NodePort",
        ];

        let items: Vec<ListItem> = menu_lists
            .iter()
            .map(|item| {
                let span = Span::from(item.to_owned());
                ListItem::new(span).style(Style::default())
            })
            .collect();

        let items = List::new(items).block(block);

        f.render_widget(items, chunksh[1]);
    }
}
