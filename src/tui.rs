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
    widgets::Block,
};
use ratatui_textarea::{Input, TextArea};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// App holds the state of the application
pub struct App {
    /// TextArea for user input
    input: TextArea<'static>,
    /// TextArea for displaying messages (read-only)
    messages_area: TextArea<'static>,
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

        // Initialize input TextArea
        let mut input = TextArea::default();
        input.set_block(Block::bordered().title("Prompt"));
        input.set_cursor_line_style(ratatui::style::Style::default());

        // Initialize messages TextArea (read-only)
        let mut messages_area = TextArea::default();
        messages_area.set_block(Block::bordered().title("Messages"));
        messages_area.set_cursor_line_style(ratatui::style::Style::default());

        Self {
            input,
            messages_area,
            ollama,
            history,
            sender,
            receiver,
            model_name,
            model_options,
        }
    }

    fn submit_message(&mut self) {
        let input = self.input.lines().join("\n");
        if input.is_empty() {
            return;
        }

        // Add user message to the messages area
        self.messages_area.insert_str(&format!("You: {}\n", input));

        // Clear input
        self.input.delete_line_by_end();

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
                self.messages_area.insert_str(&msg);
            }

            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => self.submit_message(),
                                KeyCode::Esc => break,
                                _ => {
                                    let input = Input::from(event);
                                    self.input.input(input);
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
            Constraint::Length(1), // Help area
            Constraint::Min(3),    // Input area (dynamic height)
            Constraint::Min(1),    // Messages area (dynamic height)
        ]);
        let [help_area, input_area, messages_area] = frame.area().layout(&layout);

        // Render help message
        frame.render_widget(
            ratatui::widgets::Paragraph::new("Roro (Press Esc to exit)"),
            help_area,
        );

        // Render input TextArea
        self.input.set_cursor_line_style(
            ratatui::style::Style::default().fg(ratatui::style::Color::Yellow),
        );
        frame.render_widget(&self.input, input_area);

        // Render messages TextArea
        self.messages_area
            .set_cursor_line_style(ratatui::style::Style::default());
        frame.render_widget(&self.messages_area, messages_area);
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
