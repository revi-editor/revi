#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Keys {
    Key(Key),
    KeyAndMod { key: Key, modk: Key },
}

impl From<crossterm::event::KeyEvent> for Keys {
    fn from(event: crossterm::event::KeyEvent) -> Self {
        let key = Key::from(event.code);
        let modk = Key::from(event.modifiers);
        if let (_, Key::Null) = (key, modk) {
            return Self::Key(key);
        }
        Self::KeyAndMod { key, modk }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    LA,
    LB,
    LC,
    LD,
    LE,
    LF,
    LG,
    LH,
    LI,
    LJ,
    LK,
    LL,
    LM,
    LN,
    LO,
    LP,
    LQ,
    LR,
    LS,
    LT,
    LU,
    LV,
    LW,
    LX,
    LY,
    LZ,
    UA,
    UB,
    UC,
    UD,
    UE,
    UF,
    UG,
    UH,
    UI,
    UJ,
    UK,
    UL,
    UM,
    UN,
    UO,
    UP,
    UQ,
    UR,
    US,
    UT,
    UU,
    UV,
    UW,
    UX,
    UY,
    UZ,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Esc,
    Ctrl,
    Alt,
    Shift,
    Space,
    Null,
    Colon,
    SimiColon,
    Caret,
    Char(char),
}
impl From<u8> for Key {
    fn from(num: u8) -> Self {
        match num {
            1 => Self::F1,
            2 => Self::F2,
            3 => Self::F3,
            4 => Self::F4,
            5 => Self::F5,
            6 => Self::F6,
            7 => Self::F7,
            8 => Self::F8,
            9 => Self::F9,
            10 => Self::F10,
            11 => Self::F11,
            _ => Self::F12,
        }
    }
}
impl From<&str> for Key {
    fn from(c: &str) -> Self {
        match c.to_lowercase().as_str() {
            "ctrl" => Key::Ctrl,
            "alt" => Key::Alt,
            "space" => Key::Space,
            "esc" => Key::Esc,
            "enter" => Key::Enter,
            "backspace" => Key::Backspace,
            "left" => Key::Left,
            "right" => Key::Right,
            "up" => Key::Up,
            "down" => Key::Down,
            "home" => Key::Home,
            "end" => Key::End,
            "pageup" => Key::PageUp,
            "pagedown" => Key::PageDown,
            "tab" => Key::Tab,
            "backtab" => Key::BackTab,
            "delete" => Key::Delete,
            "insert" => Key::Insert,
            "f1" => Self::from(1),
            "f2" => Self::from(2),
            "f3" => Self::from(3),
            "f4" => Self::from(4),
            "f5" => Self::from(5),
            "f6" => Self::from(6),
            "f7" => Self::from(7),
            "f8" => Self::from(8),
            "f9" => Self::from(9),
            "f10" => Self::from(10),
            "f11" => Self::from(11),
            "f12" => Self::from(12),
            _ => Key::Null,
        }
    }
}
impl From<char> for Key {
    fn from(c: char) -> Self {
        match c {
            'a' => Self::LA,
            'b' => Self::LB,
            'c' => Self::LC,
            'd' => Self::LD,
            'e' => Self::LE,
            'f' => Self::LF,
            'g' => Self::LG,
            'h' => Self::LH,
            'i' => Self::LI,
            'j' => Self::LJ,
            'k' => Self::LK,
            'l' => Self::LL,
            'm' => Self::LM,
            'n' => Self::LN,
            'o' => Self::LO,
            'p' => Self::LP,
            'q' => Self::LQ,
            'r' => Self::LR,
            's' => Self::LS,
            't' => Self::LT,
            'u' => Self::LU,
            'v' => Self::LV,
            'w' => Self::LW,
            'x' => Self::LX,
            'y' => Self::LY,
            'z' => Self::LZ,
            'A' => Self::UA,
            'B' => Self::UB,
            'C' => Self::UC,
            'D' => Self::UD,
            'E' => Self::UE,
            'F' => Self::UF,
            'G' => Self::UG,
            'H' => Self::UH,
            'I' => Self::UI,
            'J' => Self::UJ,
            'K' => Self::UK,
            'L' => Self::UL,
            'M' => Self::UM,
            'N' => Self::UN,
            'O' => Self::UO,
            'P' => Self::UP,
            'Q' => Self::UQ,
            'R' => Self::UR,
            'S' => Self::US,
            'T' => Self::UT,
            'U' => Self::UU,
            'V' => Self::UV,
            'W' => Self::UW,
            'X' => Self::UX,
            'Y' => Self::UY,
            'Z' => Self::UZ,
            '0' => Self::N0,
            '1' => Self::N1,
            '2' => Self::N2,
            '3' => Self::N3,
            '4' => Self::N4,
            '5' => Self::N5,
            '6' => Self::N6,
            '7' => Self::N7,
            '8' => Self::N8,
            '9' => Self::N9,
            ':' => Self::Colon,
            ';' => Self::SimiColon,
            '^' => Self::Caret,
            _ => Self::Char(c),
        }
    }
}

impl From<crossterm::event::KeyCode> for Key {
    fn from(key: crossterm::event::KeyCode) -> Self {
        use crossterm::event::KeyCode;
        match key {
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Enter => Key::Enter,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::Tab => Key::Tab,
            KeyCode::BackTab => Key::BackTab,
            KeyCode::Delete => Key::Delete,
            KeyCode::Insert => Key::Insert,
            KeyCode::F(num) => Self::from(num),
            KeyCode::Char(c) => Self::from(c),
            KeyCode::Null => Key::Null,
            KeyCode::Esc => Key::Esc,
        }
    }
}

impl Key {
    #[must_use]
    pub fn try_digit(self) -> Option<usize> {
        match self {
            Self::N0 => Some(0),
            Self::N1 => Some(1),
            Self::N2 => Some(2),
            Self::N3 => Some(3),
            Self::N4 => Some(4),
            Self::N5 => Some(5),
            Self::N6 => Some(6),
            Self::N7 => Some(7),
            Self::N8 => Some(8),
            Self::N9 => Some(9),
            _ => None,
        }
    }

    #[must_use]
    pub fn from_event(key: crossterm::event::KeyEvent) -> (Self, Self) {
        (Self::from(key.code), Self::from(key.modifiers))
    }

    #[must_use]
    pub fn as_char(self) -> char {
        match self {
            Self::LA => 'a',
            Self::LB => 'b',
            Self::LC => 'c',
            Self::LD => 'd',
            Self::LE => 'e',
            Self::LF => 'f',
            Self::LG => 'g',
            Self::LH => 'h',
            Self::LI => 'i',
            Self::LJ => 'j',
            Self::LK => 'k',
            Self::LL => 'l',
            Self::LM => 'm',
            Self::LN => 'n',
            Self::LO => 'o',
            Self::LP => 'p',
            Self::LQ => 'q',
            Self::LR => 'r',
            Self::LS => 's',
            Self::LT => 't',
            Self::LU => 'u',
            Self::LV => 'v',
            Self::LW => 'w',
            Self::LX => 'x',
            Self::LY => 'y',
            Self::LZ => 'z',
            Self::UA => 'A',
            Self::UB => 'B',
            Self::UC => 'C',
            Self::UD => 'D',
            Self::UE => 'E',
            Self::UF => 'F',
            Self::UG => 'G',
            Self::UH => 'H',
            Self::UI => 'I',
            Self::UJ => 'J',
            Self::UK => 'K',
            Self::UL => 'L',
            Self::UM => 'M',
            Self::UN => 'N',
            Self::UO => 'O',
            Self::UP => 'P',
            Self::UQ => 'Q',
            Self::UR => 'R',
            Self::US => 'S',
            Self::UT => 'T',
            Self::UU => 'U',
            Self::UV => 'V',
            Self::UW => 'W',
            Self::UX => 'X',
            Self::UY => 'Y',
            Self::UZ => 'Z',
            Self::N0 => '0',
            Self::N1 => '1',
            Self::N2 => '2',
            Self::N3 => '3',
            Self::N4 => '4',
            Self::N5 => '5',
            Self::N6 => '6',
            Self::N7 => '7',
            Self::N8 => '8',
            Self::N9 => '9',
            Self::Colon => ':',
            Self::SimiColon => ';',
            Self::Char(c) => c,
            _ => '\0',
        }
    }
}

impl From<crossterm::event::KeyModifiers> for Key {
    fn from(key: crossterm::event::KeyModifiers) -> Self {
        use crossterm::event::KeyModifiers;
        if key.intersects(KeyModifiers::ALT) {
            return Self::Alt;
        }
        if key.intersects(KeyModifiers::CONTROL) {
            return Self::Ctrl;
        }
        // if key.intersects(KeyModifiers::SHIFT) {
        //     return Self::Shift;
        // }
        Self::Null
    }
}

#[test]
fn test_from_crossterm_key_to_revi_key_colon() {
    use crossterm::event::{KeyCode, KeyModifiers};
    let (k1, k2) = (KeyCode::Char(':'), KeyModifiers::SHIFT);
    let left = (Key::from(k1), Key::from(k2));
    assert_eq!(left, (Key::Colon, Key::Null));
}

#[test]
fn test_from_crossterm_key_to_revi_key_upper_a() {
    use crossterm::event::{KeyCode, KeyModifiers};
    let (k1, k2) = (KeyCode::Char('A'), KeyModifiers::SHIFT);
    let left = (Key::from(k1), Key::from(k2));
    assert_eq!(left, (Key::UA, Key::Null));
}

#[macro_export]
macro_rules! keys {
    ( $( $x:ident $(($($args:expr),*))? ),* ) => {
        {
            vec![$(Key::$x $(($($args),*))? ),*]
        }
    };
}
