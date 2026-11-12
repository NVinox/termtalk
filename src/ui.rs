use std::io::{self, stdout};
use std::rc::Rc;
use std::time::Duration;

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use tokio::sync::mpsc::{Receiver, Sender};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatEvent {
    UserMessage { username: String, text: String },
    Join { username: String },
}

pub struct UI {
    username: String,
    tx_to_network: Sender<String>,
    tr_from_network: Receiver<String>,
    messages: Vec<ChatEvent>,
}

impl UI {
    pub fn new(
        tx_to_network: Sender<String>,
        tr_from_network: Receiver<String>,
        username: impl Into<String>,
    ) -> Self {
        Self {
            username: username.into(),
            tx_to_network,
            tr_from_network,
            messages: vec![],
        }
    }

    pub async fn create(mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut input = String::new();
        let mut list_state = ListState::default();

        let tx = self.tx_to_network.clone();
        let join_event = ChatEvent::Join {
            username: self.username.clone(),
        };
        let connect_message = serde_json::to_string(&join_event).unwrap();
        let _ = tx.send(connect_message).await;

        loop {
            while let Ok(message) = self.tr_from_network.try_recv() {
                let event = serde_json::from_str(&message).unwrap();

                self.messages.push(event);

                let last_index = self.messages.len().saturating_sub(1);
                list_state.select(Some(last_index));
            }

            terminal.draw(|frame| {
                let chunks = self.chunks(frame);
                let messages_list = self.messages_list();
                let input_paragraph = self.input_paragraph(&input);

                frame.render_stateful_widget(messages_list, chunks[0], &mut list_state);
                frame.render_widget(input_paragraph, chunks[1]);
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter => {
                            let message = input.clone();
                            let trimed = message.trim();

                            if !trimed.is_empty() {
                                let event = ChatEvent::UserMessage {
                                    username: self.username.clone(),
                                    text: trimed.to_string(),
                                };
                                let message = serde_json::to_string(&event).unwrap();
                                let tx = self.tx_to_network.clone();

                                tokio::spawn(async move {
                                    let _ = tx.send(message).await;
                                });

                                input.clear();
                            }
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Down => list_state.select_next(),
                        KeyCode::Up => list_state.select_previous(),
                        KeyCode::Char(c) => {
                            input.push(c);
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn chunks(&self, frame: &Frame) -> Rc<[Rect]> {
        Layout::default()
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area())
    }

    fn messages_list(&self) -> List<'_> {
        let list_items = self.list_items();

        List::new(list_items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray))
    }

    fn list_items(&self) -> Vec<ListItem<'_>> {
        self.messages
            .iter()
            .map(|msg| self.list_item(msg))
            .collect()
    }

    fn list_item(&self, event: &ChatEvent) -> ListItem<'_> {
        match event {
            ChatEvent::UserMessage { username, text } => {
                let line = Line::from(format!("{}: {}", username, text));
                ListItem::new(line)
            }
            ChatEvent::Join { username } => {
                let line = Line::from(format!("User {} is connected", username))
                    .green()
                    .italic();
                ListItem::new(line)
            }
        }
    }

    fn input_paragraph<'a>(&self, input: &'a str) -> Paragraph<'a> {
        Paragraph::new(input).block(Block::default().borders(Borders::ALL))
    }
}
