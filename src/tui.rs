mod chat;

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
    help_view: Paragraph<'static>,
    prompt_view: TextArea<'static>,
    chat_view: chat::ChatView<'static>,
    layout: Layout,
}

impl TuiApp {
    pub fn new(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<ChatMessage>) -> Self {
        let layout = Layout::vertical([
            Constraint::Length(1), // Help area
            Constraint::Length(5), // Prompt area
            Constraint::Min(7),    // Chat area
        ]);

        let help_view = Paragraph::new("Roro (Press Esc to exit)");

        let mut prompt_view = TextArea::default();
        prompt_view.set_wrap_mode(WrapMode::WordOrGlyph);
        prompt_view.set_block(Block::bordered().title("Prompt"));
        prompt_view.set_cursor_line_style(Style::default().fg(Color::Yellow));

        Self {
            sender,
            receiver,
            help_view,
            prompt_view,
            chat_view: chat::ChatView::new(),
            layout,
        }
    }

    fn submit_message(&mut self) {
        let input = self.prompt_view.lines().join("\n");
        if input.is_empty() {
            return;
        }

        if !self.chat_view.is_empty() {
            self.chat_view.insert_newline();
        }
        self.chat_view.insert_str(&format!("You ~ {input}"));
        self.chat_view.insert_newline();
        self.prompt_view.clear();

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
                        self.chat_view.insert_str("Thinking > ");
                        is_thinking = true;
                    }
                    self.chat_view.last_mut().map(|s| s.push_str(&thinking_content));
                } else {
                    if !is_assistant_prefix_added {
                        self.chat_view.insert_str("Assistant > ");
                        is_assistant_prefix_added = true;
                    }

                    self.chat_view.last_mut().map(|s| s.push_str(&msg.content));
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

                                KeyCode::Up if key.modifiers.contains(KeyModifiers::SHIFT) => self.chat_view.scroll_up(),
                                KeyCode::Down if key.modifiers.contains(KeyModifiers::SHIFT) => self.chat_view.scroll_down(),
                                KeyCode::Up => self.prompt_view.scroll((-1, 0)),
                                KeyCode::Down => self.prompt_view.scroll((1, 0)),
                                KeyCode::PageUp => self.chat_view.scroll_page_up(),
                                KeyCode::PageDown => self.chat_view.scroll_page_down(),

                                _ => {
                                    let input = Input::from(event);
                                    self.prompt_view.input(input);
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

        frame.render_widget(&self.help_view, help_area);
        frame.render_widget(&self.prompt_view, prompt_area);
        frame.render_widget(&self.chat_view, chat_area);
    }
}
