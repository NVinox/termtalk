use std::io::{self, stdout};
use std::time::Duration;

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use tokio::sync::mpsc::{Receiver, Sender};

pub struct UI {
    username: String,
    tx_to_network: Sender<String>,
    tr_from_network: Receiver<String>,
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
        }
    }

    pub async fn create(mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut input = String::new();
        let mut messages: Vec<String> = vec![];

        loop {
            while let Ok(message) = self.tr_from_network.try_recv() {
                messages.push(message);
            }

            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(frame.area());
                let list_items: Vec<ListItem> = messages
                    .iter()
                    .map(|msg| ListItem::new(msg.as_str()))
                    .collect();
                let messages_list =
                    List::new(list_items).block(Block::default().borders(Borders::ALL));
                let input_paragraph =
                    Paragraph::new(input.as_str()).block(Block::default().borders(Borders::ALL));

                frame.render_widget(messages_list, chunks[0]);
                frame.render_widget(input_paragraph, chunks[1]);
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter => {
                            let message = format!("{}: {}", self.username, input.clone().trim());
                            if !message.is_empty() {
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
}
