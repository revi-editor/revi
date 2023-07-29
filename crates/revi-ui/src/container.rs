use crate::layout::{Pos, Rect, Size, Stack};
use crate::widget::{BoxWidget, Widget};
use std::io::Stdout;
#[derive(Debug, Default)]
pub struct Container {
    pub bounds: Rect,
    stack: Stack,
    children: Vec<BoxWidget>,
    vert_center: bool,
    comment: Option<String>,
}

impl Container {
    pub fn new(bounds: Rect, stack: Stack) -> Self {
        Self {
            bounds,
            stack,
            children: Vec::new(),
            vert_center: false,
            comment: None,
        }
    }

    pub fn with_bounds(mut self, rect: Rect) -> Self {
        self.bounds = rect;
        self
    }

    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    pub fn vcenter(mut self) -> Self {
        self.vert_center = true;
        self
    }

    pub fn stack(mut self, stack: Stack) -> Self {
        self.stack = stack;
        self
    }

    pub fn push(mut self, widget: impl Into<BoxWidget>) -> Self {
        self.children.push(widget.into());
        self
    }
}

impl Widget for Container {
    fn x(&self) -> u16 {
        self.bounds.x()
    }

    fn y(&self) -> u16 {
        self.bounds.y()
    }

    fn width(&self) -> u16 {
        self.bounds.width()
    }

    fn height(&self) -> u16 {
        self.bounds.height()
    }

    fn draw(&self, stdout: &mut Stdout, bounds: Rect) {
        for (widget, mut wbounds) in self.children.iter().zip(generate_layout(
            bounds,
            self.bounds,
            &self.children,
            self.stack,
        )) {
            if let Stack::Vertically = self.stack {
                let height = self.height() / 2;
                let children_len = self.children.len() as u16;
                let content_height = children_len / 2;
                let y_offset = height.saturating_sub(content_height); // + (idx as u16);
                wbounds.y += y_offset * self.vert_center as u16;
            }
            widget.draw(stdout, wbounds);
        }
    }
    fn debug_name(&self) -> String {
        self.comment.clone().unwrap_or_default()
    }
}

fn generate_layout(root: Rect, current: Rect, children: &[BoxWidget], stack: Stack) -> Vec<Rect> {
    children.iter().fold(vec![], |mut acc, child| {
        let last = acc.last().cloned().unwrap_or_default();
        let x = match stack {
            Stack::Vertically => current.x() + child.x() + root.x(),
            Stack::Horizontally => current.x() + child.x() + last.width() + last.x() + root.x(),
        };
        let y = match stack {
            Stack::Vertically => current.y() + child.y() + last.height() + last.y() + root.y(),
            Stack::Horizontally => current.y() + child.y() + root.y(),
        };
        let width = match stack {
            // NOTE: child should be priority if Shrinking is in effect
            Stack::Vertically => child.width().max(current.width()).min(root.width()),
            Stack::Horizontally => child
                .width()
                .min(current.width() - last.width())
                .min(root.width()),
        };
        let height = match stack {
            Stack::Vertically => child
                .height()
                .min(current.height() - last.height())
                .min(root.height()),
            Stack::Horizontally => child.height().min(current.height()).min(root.height()),
        };
        let size = Size::new(width, height);
        let pos = Pos::new(x, y);
        let rect = Rect::with_position(pos, size);
        acc.push(rect);
        acc
    })
}

impl From<Container> for BoxWidget {
    fn from(container: Container) -> Self {
        BoxWidget::new(container)
    }
}

impl From<Rect> for Container {
    fn from(rect: Rect) -> Self {
        Self::new(rect, Stack::Vertically)
    }
}

impl From<(Rect, Stack)> for Container {
    fn from((rect, stack): (Rect, Stack)) -> Self {
        Self::new(rect, stack)
    }
}
