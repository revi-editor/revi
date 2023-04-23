use revi_ui::Keys;
#[derive(Debug)]
pub struct KeyParser {
    multiplier: usize,
    keys: Vec<Keys>,
    idx: usize,
}

impl Default for KeyParser {
    fn default() -> Self {
        Self {
            multiplier: 1,
            keys: Vec::with_capacity(20),
            idx: 0,
        }
    }
}

impl KeyParser {
    pub fn push(&mut self, keys: Keys) {
        if keys.is_null() {
            return;
        }
        if self.idx >= self.keys.len() {
            self.idx += 1;
            self.keys.push(keys);
            return;
        }
        self.keys[self.idx] = keys;
        self.idx += 1;
    }

    pub fn clear(&mut self) {
        self.idx = 0;
        self.multiplier = 1;
    }

    pub fn get_keys(&self) -> &[Keys] {
        &self.keys[..self.idx]
    }
}
