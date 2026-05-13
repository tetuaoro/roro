use ratatui::style::Color;
use ratatui::widgets::Wrap;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

pub struct ChatView<'a> {
    border_box: Block<'a>,
    scroll: u16,
    max_scroll: u16,
    content: Vec<String>,
}

impl<'a> ChatView<'a> {
    pub fn new() -> Self {
        Self {
            border_box: Block::bordered().title("Chat"),
            scroll: 0,
            max_scroll: 0,
            content: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn insert_newline(&mut self) {
        self.content.push(String::new());
        self.max_scroll = self.content.len().saturating_sub(1) as u16;
    }

    pub fn insert_str(&mut self, str: &str) {
        self.content.push(str.to_string());
        self.max_scroll = self.content.len().saturating_sub(1) as u16;
    }

    pub fn last_mut(&mut self) -> Option<&mut String> {
        self.content.last_mut()
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll = (self.scroll + 1).min(self.max_scroll);
    }

    pub fn scroll_page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(self.max_scroll);
    }

    pub fn scroll_page_down(&mut self) {
        self.scroll = self.scroll.saturating_add(self.max_scroll - self.scroll);
    }
}

impl Widget for ChatView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &ChatView<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let inner_rect = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),   // sub border
            height: area.height.saturating_sub(2), // sub border
        };

        let paragraph_lines = self
            .content
            .iter()
            .skip(self.scroll as usize)
            .take(inner_rect.height as usize)
            .map(|line| Line::from(Span::raw(line.clone())))
            .collect::<Vec<_>>();

        let paragraph = Paragraph::new(paragraph_lines)
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::Green));

        self.border_box.clone().render(area, buf);
        paragraph.render(inner_rect, buf);
    }
}
