use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};
use std::cell::RefCell;

pub struct ChatView<'a> {
    scroll: u16,
    content: String,
    height: RefCell<u16>,
    border_box: Block<'a>,
}

impl<'a> ChatView<'a> {
    pub fn new() -> Self {
        Self {
            scroll: 0,
            content: String::new(),
            height: RefCell::new(0),
            border_box: Block::bordered().title("Chat"),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    pub fn insert_newline(&mut self) {
        self.content.push_str("  \n");
    }

    pub fn insert_str(&mut self, s: &str) {
        self.content.push_str(s);
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub fn scroll_page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(*self.height.borrow());
    }

    pub fn scroll_page_down(&mut self) {
        self.scroll = self.scroll.saturating_add(*self.height.borrow());
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

        let text_wrap = textwrap::wrap(&self.content, inner_rect.width as usize);
        let input = text_wrap.join("  \n");
        let text = tui_markdown::from_str(&input);

        let height = text.lines.len().saturating_sub(inner_rect.height as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        let mut scrollbar_state = ScrollbarState::new(height).position((self.scroll as usize).min(height));

        self.border_box.clone().render(area, buf);
        text.render(inner_rect, buf);
        scrollbar.render(inner_rect, buf, &mut scrollbar_state);
    }
}
