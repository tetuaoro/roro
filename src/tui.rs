use color_eyre::Result;
use ollama_rs::generation::chat::ChatMessage;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use ratatui_textarea::{Input, TextArea, WrapMode};
use tokio::sync::mpsc;

pub struct TuiApp {
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<ChatMessage>,
    help_area: Paragraph<'static>,
    prompt_area: TextArea<'static>,
    chat_area: TextArea<'static>,
    layout: Layout,
}

impl TuiApp {
    pub fn new(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<ChatMessage>) -> Self {
        let layout = Layout::vertical([
            Constraint::Length(1), // Help area
            Constraint::Length(5), // Prompt area
            Constraint::Min(1),    // Chat area
        ]);

        let help_area = Paragraph::new("Roro (Press Esc to exit)");

        let mut prompt_area = TextArea::default();
        prompt_area.set_wrap_mode(WrapMode::WordOrGlyph);
        prompt_area.set_block(Block::bordered().title("Prompt"));
        prompt_area.set_cursor_line_style(Style::default().fg(Color::Yellow));

        let mut chat_area = TextArea::default();
        chat_area.set_wrap_mode(WrapMode::WordOrGlyph);
        chat_area.set_block(Block::bordered().title("Chat"));
        chat_area.set_cursor_line_style(Style::default());
        chat_area.set_cursor_style(Style::default().fg(Color::Reset));

        Self {
            sender,
            receiver,
            help_area,
            prompt_area,
            chat_area,
            layout,
        }
    }

    fn submit_message(&mut self) {
        let input = self.prompt_area.lines().join("\n");
        if input.is_empty() {
            return;
        }

        if !self.chat_area.is_empty() {
            self.chat_area.insert_newline();
        }
        self.chat_area.insert_str(&format!("You ~ {input}"));
        self.chat_area.insert_newline();
        self.prompt_area.clear();

        let sender = self.sender.clone();
        tokio::spawn(async move {
            let _ = sender.send(input).await;
        });
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut is_thinking = false;
        let mut is_assistant_prefix_added = false;

        loop {
            while let Ok(msg) = self.receiver.try_recv() {
                if let Some(thinking_content) = msg.thinking {
                    if !is_thinking {
                        self.chat_area.insert_str("Thinking > ");
                        is_thinking = true;
                    }
                    self.chat_area.insert_str(&thinking_content);
                } else {
                    if !is_assistant_prefix_added {
                        if is_thinking {
                            self.chat_area.insert_newline();
                        }
                        self.chat_area.insert_str("Assistant > ");
                        is_assistant_prefix_added = true;
                    }
                    self.chat_area.insert_str(&msg.content);
                }
            }

            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Ok(event) = event::read() {
                    if let Event::Key(key) = event {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Enter => {
                                    is_thinking = false;
                                    is_assistant_prefix_added = false;
                                    self.submit_message();
                                }
                                KeyCode::Esc => break,
                                KeyCode::Up => {
                                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.chat_area.scroll((-1, 0));
                                    } else {
                                        self.prompt_area.scroll((-1, 0));
                                    }
                                }
                                KeyCode::Down => {
                                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                                        self.chat_area.scroll((1, 0));
                                    } else {
                                        self.prompt_area.scroll((1, 0));
                                    }
                                }
                                KeyCode::PageUp => {
                                    self.chat_area.scroll((-10, 0));
                                }
                                KeyCode::PageDown => {
                                    self.chat_area.scroll((10, 0));
                                }
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
        let [help_area, prompt_area, chat_area] = frame.area().layout(&self.layout);
        frame.render_widget(&self.help_area, help_area);
        frame.render_widget(&self.prompt_area, prompt_area);
        frame.render_widget(&self.chat_area, chat_area);
    }
}
