use std::sync::{Arc, Mutex};

use color_eyre::Result;

use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
    models::ModelOptions,
};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph, Wrap},
};

use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area
    character_index: usize,
    /// History of recorded messages
    messages: Vec<String>,
    /// Ollama client
    ollama: Ollama,
    /// Shared chat history
    history: Arc<Mutex<Vec<ChatMessage>>>,
    /// Sender to communicate with the TUI thread
    sender: mpsc::Sender<String>,
    /// Receiver to get responses from Ollama
    receiver: mpsc::Receiver<String>,
    /// Model name for Ollama
    model_name: String,
    /// Model options
    model_options: ModelOptions,
}

impl App {
    pub fn new(
        ollama: Ollama,
        history: Arc<Mutex<Vec<ChatMessage>>>,
        model_name: String,
        model_options: ModelOptions,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(32);
        Self {
            input: String::new(),
            messages: Vec::new(),
            character_index: 0,
            ollama,
            history,
            sender,
            receiver,
            model_name,
            model_options,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let input = self.input.clone();
        self.messages.push(format!("You: {}", input));
        self.input.clear();
        self.reset_cursor();

        let ollama = self.ollama.clone();
        let history = self.history.clone();
        let sender = self.sender.clone();
        let model_name = self.model_name.clone();
        let model_options = self.model_options.clone();

        tokio::spawn(async move {
            {
                let mut history_lock = history.lock().unwrap();
                history_lock.push(ChatMessage::user(input.clone()));
            }

            let stream = ollama
                .send_chat_messages_with_history_stream(
                    history.clone(),
                    ChatMessageRequest::new(model_name, vec![ChatMessage::user(input)])
                        .tools(vec![])
                        .options(model_options),
                )
                .await;

            match stream {
                Ok(mut stream) => {
                    let mut full_response = String::new();
                    while let Some(res) = stream.next().await {
                        match res {
                            Ok(res) => {
                                let content = res.message.content;
                                full_response.push_str(&content);
                                if let Err(e) = sender.send(content).await {
                                    eprintln!("Failed to send response chunk: {}", e);
                                    break;
                                }
                            }
                            Err(_) => {
                                eprintln!("Error during stream");
                                break;
                            }
                        }
                    }
                    {
                        let mut history_lock = history.lock().unwrap();
                        history_lock.push(ChatMessage::assistant(full_response));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to start stream: {}", e);
                    let _ = sender.send(format!("Error: {}", e)).await;
                }
            }
        });
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            // Process incoming messages from Ollama
            while let Ok(msg) = self.receiver.try_recv() {
                if let Some(last_msg) = self.messages.last_mut() {
                    if last_msg.starts_with("Assistant:") {
                        *last_msg = format!("{}{}", last_msg, msg);
                    } else {
                        self.messages.push(format!("Assistant: {}", msg));
                    }
                } else {
                    self.messages.push(format!("Assistant: {}", msg));
                }
            }

            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => self.submit_message(),
                                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                                KeyCode::Backspace => self.delete_char(),
                                KeyCode::Left => self.move_cursor_left(),
                                KeyCode::Right => self.move_cursor_right(),
                                KeyCode::Esc => break,
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = frame.area().layout(&layout);

        let text = Text::from(Line::from("Roro (Press Esc to exit)")).patch_style(Style::default());
        let help_message = Paragraph::new(text);

        let prompt = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::bordered().title("Prompt"))
            .wrap(Wrap { trim: true });

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .map(|m| {
                let content = Line::from(Span::raw(m.clone()));
                ListItem::new(content)
            })
            .collect();
        let messages_list = List::new(messages).block(Block::bordered().title("Messages"));

        frame.render_widget(help_message, help_area);
        frame.render_widget(prompt, input_area);
        frame.set_cursor_position(Position::new(
            input_area.x + self.character_index as u16 + 1,
            input_area.y + 1,
        ));
        frame.render_widget(messages_list, messages_area);
    }
}

pub fn run(
    ollama: Ollama,
    history: Arc<Mutex<Vec<ChatMessage>>>,
    model_name: String,
    model_options: ModelOptions,
) -> Result<()> {
    color_eyre::install()?;
    let app = App::new(ollama, history, model_name, model_options);
    ratatui::run(|terminal| app.run(terminal))
}
