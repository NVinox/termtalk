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
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use tokio::sync::mpsc::{Receiver, Sender};

use serde::{Deserialize, Serialize};
use serde_json;

const CHAT_TITLE: &'static str = " Chat ";
const INPUT_TITLE: &'static str = " Message ";

const EVENT_POOL_DURATION: u64 = 16;
const INPUT_MIN_LINES: u16 = 3;
const INPUT_MAX_LINES: u16 = 8;

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
    input: String,
    autoscroll: bool,
    total_lines: usize,
    list_state: ListState,
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
            input: String::new(),
            autoscroll: true,
            total_lines: 0,
            list_state: ListState::default(),
        }
    }

    pub async fn create(mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

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
            }

            terminal.draw(|frame| {
                let chunks = Self::chunks(&self.input, frame);
                let input_paragraph = Self::input_paragraph(&self.input);
                let current_width = chunks[0].width.saturating_sub(2).max(1) as usize;
                let list_items = Self::list_items(&self.messages, current_width);

                self.total_lines = list_items.len();

                if self.autoscroll && !list_items.is_empty() {
                    self.list_state
                        .select(Some(list_items.len().saturating_sub(1)));
                }

                let messages_list = List::new(list_items)
                    .block(Block::default().borders(Borders::ALL).title(CHAT_TITLE))
                    .highlight_style(Style::default().bg(Color::DarkGray));

                frame.render_stateful_widget(messages_list, chunks[0], &mut self.list_state);
                frame.render_widget(input_paragraph, chunks[1]);
            })?;

            if event::poll(Duration::from_millis(EVENT_POOL_DURATION))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter => {
                            self.send_message();
                        }
                        KeyCode::Backspace => {
                            self.delete_character();
                        }
                        KeyCode::Up => {
                            self.scroll_up();
                        }
                        KeyCode::Down => {
                            self.scroll_down();
                        }
                        KeyCode::Char(c) => {
                            self.add_character(c);
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

    fn chunks(input: &str, frame: &Frame) -> Rc<[Rect]> {
        let inner_width = frame.area().width.saturating_sub(2).max(1) as usize;
        let lines_count = textwrap::wrap(input, inner_width).len() as u16;
        let input_height = (lines_count + 2).clamp(INPUT_MIN_LINES, INPUT_MAX_LINES);

        Layout::default()
            .constraints([Constraint::Min(0), Constraint::Length(input_height)])
            .split(frame.area())
    }

    fn list_items(messages: &[ChatEvent], width: usize) -> Vec<ListItem<'_>> {
        messages
            .iter()
            .flat_map(|event| Self::list_item(event, width))
            .collect()
    }

    fn list_item(event: &ChatEvent, width: usize) -> Vec<ListItem<'_>> {
        let options = textwrap::Options::new(width).break_words(true);

        match event {
            ChatEvent::UserMessage { username, text } => {
                let full_text = format!("{}: {}", username, text);
                textwrap::wrap(&full_text, &options)
                    .into_iter()
                    .map(|line| ListItem::new(Line::from(line.into_owned())))
                    .collect::<Vec<_>>()
            }
            ChatEvent::Join { username } => {
                let line = Line::from(format!("User {} is connected", username))
                    .green()
                    .italic();
                vec![ListItem::new(line)]
            }
        }
    }

    fn input_paragraph(input: &str) -> Paragraph<'_> {
        Paragraph::new(input)
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).title(INPUT_TITLE))
    }

    fn send_message(&mut self) {
        let message = self.input.clone();
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

            self.input.clear();
        }
    }

    fn scroll_up(&mut self) {
        self.list_state.select_previous();
        self.autoscroll_state();
    }

    fn scroll_down(&mut self) {
        self.list_state.select_next();
        self.autoscroll_state();
    }

    fn autoscroll_state(&mut self) {
        if let Some(selected_index) = self.list_state.selected() {
            let last_index = self.total_lines.saturating_sub(1);

            if selected_index >= last_index {
                self.autoscroll = true;
            } else {
                self.autoscroll = false;
            }
        }
    }

    fn delete_character(&mut self) {
        self.input.pop();
    }

    fn add_character(&mut self, ch: char) {
        self.input.push(ch);
    }
}
