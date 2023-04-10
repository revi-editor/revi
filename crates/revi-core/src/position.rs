use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn new_u16(x: u16, y: u16) -> Self {
        Self {
            x: x as usize,
            y: y as usize,
        }
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    pub fn add_to_x(&mut self, x: usize) {
        self.x = self.x.saturating_add(x);
    }

    pub fn sub_to_x(&mut self, x: usize) {
        self.x = self.x.saturating_sub(x);
    }

    pub fn add_to_y(&mut self, y: usize) {
        self.y = self.y.saturating_add(y);
    }

    pub fn sub_to_y(&mut self, y: usize) {
        self.y = self.y.saturating_sub(y);
    }

    #[must_use]
    pub fn as_usize(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    #[must_use]
    pub fn as_usize_x(&self) -> usize {
        self.x
    }

    #[must_use]
    pub fn as_usize_y(&self) -> usize {
        self.y
    }

    #[must_use]
    pub fn as_u16(&self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }

    #[must_use]
    pub fn as_u16_x(&self) -> u16 {
        self.x as u16
    }

    #[must_use]
    pub fn as_u16_y(&self) -> u16 {
        self.y as u16
    }

    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Self::new_u16(x, y)
    }
}

impl From<&(u16, u16)> for Position {
    fn from((x, y): &(u16, u16)) -> Self {
        Self::new_u16(*x, *y)
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x as usize, y as usize)
    }
}

impl From<&(i32, i32)> for Position {
    fn from((x, y): &(i32, i32)) -> Self {
        Self::new(*x as usize, *y as usize)
    }
}
