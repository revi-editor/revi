#![allow(dead_code)]
#[derive(Debug, Default)]
struct Branch {
    value: char,
    end: bool,
    child: Vec<Branch>,
}

impl Branch {
    fn new(word: &[char]) -> Self {
        let child = if word.len() > 1 {
            vec![Branch::new(&word[1..])]
        } else {
            vec![]
        };
        Self {
            value: word[0],
            end: word.len() == 1,
            child,
        }
    }

    fn has_value(&self, value_of: char) -> bool {
        self.value == value_of
    }

    fn insert(&mut self, word: &[char]) -> bool {
        if self.value == word[0] && word.len() > 1 {
            for child in self.child.iter_mut() {
                if child.has_value(word[1]) {
                    return child.insert(&word[1..]);
                }
            }
            self.child.push(Branch::new(&word[1..]));
        } else if self.value == word[0] {
            self.end = true;
            return true;
        }
        false
    }

    fn _contains(&self, word: &[char]) -> bool {
        for child in self.child.iter() {
            if child.has_value(word[0]) {
                if word.len() > 1 {
                    return child._contains(&word[1..]);
                } else if child.end && word.len() == 1 {
                    return true;
                }
            }
        }
        false
    }

    fn get_rest_branch(&self, mut last: String, word_list: &mut Vec<String>) {
        if self.end {
            word_list.push(format!("{last}{}", self.value));
        }
        last.push(self.value);
        for child in self.child.iter() {
            child.get_rest_branch(last.clone(), word_list);
        }
    }

    fn look_up(&self, word: &[char], mut last: String, word_list: &mut Vec<String>) {
        if self.end {
            word_list.push(format!("{last}{}", self.value));
        }
        last.push(self.value);
        for child in self.child.iter() {
            if word.len() > 1 && child.has_value(word[0]) {
                child.look_up(&word[1..], last.to_string(), word_list);
            } else if child.has_value(word[0]) {
                child.get_rest_branch(last.to_string(), word_list);
            }
        }
    }
}

// ```
// fn main() {
//     let mut tree = Trie::default();
//     tree.insert("struct");
//     tree.insert("foo");
//     tree.insert("bar");
//     tree.insert("structure");
//     tree.insert("string");
//     assert_eq!(tree.get_words("str"), vec!["struct", "string", "structure"]);
// }
// ```
#[derive(Debug, Default)]
pub struct Trie {
    branches: Vec<Branch>,
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }
        let chars = word.chars().collect::<Vec<char>>();
        for branch in self.branches.iter_mut() {
            if branch.has_value(chars[0]) {
                let _ = branch.insert(&chars);
                return;
            }
        }
        self.branches.push(Branch::new(&chars));
    }

    pub fn contains(&self, word: &str) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        for branch in self.branches.iter() {
            if branch.has_value(word[0]) {
                if word.len() > 1 {
                    return branch._contains(&word[1..]);
                } else if branch.end && word.len() == 1 {
                    return true;
                }
            }
        }
        false
    }

    pub fn lookup(&self, pre_fix: &str) -> Vec<String> {
        if pre_fix.is_empty() {
            return vec![];
        }
        let mut word_list = vec![];
        let word = pre_fix.chars().collect::<Vec<char>>();
        for branch in self.branches.iter() {
            if word.len() > 1 && branch.has_value(word[0]) {
                branch.look_up(&word[1..], "".to_string(), &mut word_list);
            } else if branch.has_value(word[0]) {
                branch.get_rest_branch("".to_string(), &mut word_list);
            }
        }
        word_list
    }

    pub fn get_all_words(&self) -> Vec<String> {
        let mut word_list = vec![];
        for branch in self.branches.iter() {
            branch.get_rest_branch("".to_string(), &mut word_list);
        }
        word_list
    }
}

impl From<&Vec<&str>> for Trie {
    fn from(word_list: &Vec<&str>) -> Self {
        if word_list.is_empty() {
            return Self::default();
        }
        let mut trie = Self::default();
        for word in word_list {
            trie.insert(word);
        }
        trie
    }
}

#[test]
fn get_all_words() {
    let trie = Trie::from(&vec![
        "cowboy",
        "color",
        "c",
        "cow",
        "collect",
        "hello",
        "hell",
        "has_value",
        "hey",
    ]);
    assert_eq!(
        trie.get_all_words(),
        vec![
            "c",
            "cow",
            "cowboy",
            "color",
            "collect",
            "hell",
            "hello",
            "hey",
            "has_value"
        ]
    )
}

#[test]
fn trie_return_right_branch() {
    let trie = Trie::from(&vec![
        "cowboy",
        "color",
        "c",
        "cow",
        "collect",
        "hello",
        "hell",
        "has_value",
        "hey",
    ]);
    assert_eq!(trie.lookup("ha"), vec!["has_value"]);
}
#[test]
fn trie_could_be_one_of() {
    let trie = Trie::from(&vec![
        "cowboy",
        "color",
        "c",
        "cow",
        "collect",
        "hello",
        "hell",
        "has_value",
        "hey",
    ]);
    assert_eq!(
        trie.lookup("c"),
        vec!["c", "cow", "cowboy", "color", "collect"],
    );
}
#[test]
fn trie_contains() {
    let trie = Trie::from(&vec![
        "cowboy",
        "color",
        "c",
        "cow",
        "collect",
        "hello",
        "hell",
        "has_value",
        "hey",
    ]);
    eprintln!("{:#?}", trie);
    assert!(trie.contains("cowboy"));
    assert!(trie.contains("color"));
    assert!(trie.contains("c"));
    assert!(trie.contains("cow"));
    assert!(trie.contains("cow"));
    assert!(trie.contains("collect"));
    assert!(trie.contains("hello"));
    assert!(trie.contains("hell"));
    assert!(trie.contains("has_value"));
    assert!(trie.contains("hey"));
    assert!(!trie.contains("h"));
    assert!(!trie.contains("hlasdjfa"));
    assert!(!trie.contains("cowboybe"));
}
