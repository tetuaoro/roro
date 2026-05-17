use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Widget},
};
use std::cell::RefCell;

pub struct ChatView<'a> {
    scroll: u16,
    content: String,
    height: RefCell<u16>,
    border_box: Block<'a>,
    max_scroll: RefCell<u16>,
}

impl<'a> ChatView<'a> {
    pub fn new() -> Self {
        Self {
            scroll: 0,
            content: String::new(),
            height: RefCell::new(0),
            border_box: Block::bordered().title("Chat"),
            max_scroll: RefCell::new(0),
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
        self.scroll = self.scroll.saturating_add(1).min(*self.max_scroll.borrow());
    }

    pub fn scroll_page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(*self.height.borrow());
    }

    pub fn scroll_page_down(&mut self) {
        self.scroll = self.scroll.saturating_add(*self.height.borrow()).min(*self.max_scroll.borrow());
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

        *self.height.borrow_mut() = inner_rect.height;

        let input = textwrap::wrap(&self.content, (inner_rect.width) as usize).join("  \n");
        let text = tui_markdown::from_str(&input);

        *self.max_scroll.borrow_mut() = text.lines.len() as u16;

        self.border_box.clone().render(area, buf);

        text.iter()
            .skip(self.scroll as usize)
            .take(inner_rect.height as usize)
            .enumerate()
            .for_each(|(i, line)| {
                let line_y = inner_rect.y + i as u16;
                line.render(
                    Rect {
                        x: inner_rect.x,
                        y: line_y,
                        width: inner_rect.width,
                        height: 1,
                    },
                    buf,
                );
            });
    }
}
