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
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use ratatui_textarea::{Input, TextArea, WrapMode};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// App holds the state of the application
pub struct App {
    /// TextArea for user input
    prompt_area: TextArea<'static>,
    /// TextArea for displaying messages (read-only)
    chat_area: TextArea<'static>,
    /// Ollama client
    ollama: Ollama,
    /// Shared chat history
    history: Arc<Mutex<Vec<ChatMessage>>>,
    /// Sender to communicate with the TUI thread
    sender: mpsc::Sender<ChatMessage>,
    /// Receiver to get responses from Ollama
    receiver: mpsc::Receiver<ChatMessage>,
    /// Model name for Ollama
    model_name: String,
    /// Model options
    model_options: ModelOptions,
}

impl App {
    pub fn new(ollama: Ollama, history: Arc<Mutex<Vec<ChatMessage>>>, model_name: String, model_options: ModelOptions) -> Self {
        let (sender, receiver) = mpsc::channel(32);

        // Initialize input TextArea
        let mut prompt_area = TextArea::default();
        prompt_area.set_block(Block::bordered().title("Prompt"));
        prompt_area.set_cursor_line_style(ratatui::style::Style::default());

        // Initialize messages TextArea (read-only)
        let mut chat_area = TextArea::default();
        chat_area.set_block(Block::bordered().title("Chat"));
        chat_area.set_cursor_line_style(ratatui::style::Style::default());

        Self {
            prompt_area,
            chat_area,
            ollama,
            history,
            sender,
            receiver,
            model_name,
            model_options,
        }
    }

    fn submit_message(&mut self) {
        let input = self.prompt_area.lines().join("\n");
        if input.is_empty() {
            return;
        }

        // Add user message to the messages area
        if !self.chat_area.is_empty() {
            self.chat_area.insert_newline();
        }
        self.chat_area.insert_str(&format!("You : {}", input));

        // Clear input
        self.prompt_area.clear();

        let ollama = self.ollama.clone();
        let history = self.history.clone();
        let sender = self.sender.clone();
        let model_name = self.model_name.clone();
        let model_options = self.model_options.clone();

        tokio::spawn(async move {
            let stream = ollama
                .send_chat_messages_with_history_stream(
                    history,
                    ChatMessageRequest::new(model_name, vec![ChatMessage::user(input)])
                        .tools(vec![])
                        .options(model_options),
                )
                .await;

            if let Ok(mut stream) = stream {
                while let Some(res) = stream.next().await {
                    if let Ok(response) = res {
                        _ = sender.send(response.message).await;
                    }
                }
            }
        });
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut is_thinking = false;
        let mut is_assistant_prefix_added = false;

        loop {
            while let Ok(msg) = self.receiver.try_recv() {
                if let Some(thinking_content) = msg.thinking {
                    if !is_thinking {
                        self.chat_area.insert_str("\nThinking > ");
                        is_thinking = true;
                    }
                    self.chat_area.insert_str(&thinking_content);
                }

                if !is_assistant_prefix_added {
                    self.chat_area.insert_str("\nAssistant > ");
                    is_assistant_prefix_added = true;
                }

                self.chat_area.insert_str(&msg.content);
            }

            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => {
                                    self.submit_message();
                                    is_thinking = false;
                                    is_assistant_prefix_added = false;
                                }
                                KeyCode::Esc => break,
                                _ => {
                                    let input = Input::from(event);
                                    self.prompt_area.input(input);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Length(1),       // Help area
            Constraint::Length(3),       // Prompt area (dynamic height)
            Constraint::Percentage(100), // Chat area (dynamic height)
        ]);
        let [help_area, prompt_area, chat_area] = frame.area().layout(&layout);

        // Render input TextArea
        self.prompt_area.set_wrap_mode(WrapMode::Glyph);
        self.prompt_area.set_cursor_line_style(Style::default().fg(Color::Yellow));

        // Render messages TextArea
        self.chat_area.set_wrap_mode(WrapMode::Glyph);
        self.chat_area.set_cursor_line_style(Style::default());

        // Render help message
        frame.render_widget(Paragraph::new("Roro (Press Esc to exit)"), help_area);
        frame.render_widget(&self.prompt_area, prompt_area);
        frame.render_widget(&self.chat_area, chat_area);
    }
}

pub fn run(ollama: Ollama, history: Arc<Mutex<Vec<ChatMessage>>>, model_name: String, model_options: ModelOptions) -> Result<()> {
    color_eyre::install()?;
    let app = App::new(ollama, history, model_name, model_options);
    ratatui::run(|terminal| app.run(terminal))
}
