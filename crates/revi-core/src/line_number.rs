#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum LineNumbers {
    RelativeNumber(u16),
    AbsoluteNumber(u16),
    None,
}

impl LineNumbers {
    pub fn width(&self) -> usize {
        match self {
            Self::AbsoluteNumber(w) | Self::RelativeNumber(w) => *w as usize,
            Self::None => 0,
        }
    }
}
