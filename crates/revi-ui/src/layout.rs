#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}
impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone)]
pub enum Spacing {
    Fill,
    Shrink,
}

impl Default for Spacing {
    fn default() -> Self {
        Self::Shrink
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stack {
    Vertically,
    Horizontally,
}

impl Default for Stack {
    fn default() -> Self {
        Self::Horizontally
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(size: Size) -> Self {
        Self::with_position(Pos::new(0, 0), size)
    }
    pub fn with_position(pos: Pos, size: Size) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.width,
            height: size.height,
        }
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}
