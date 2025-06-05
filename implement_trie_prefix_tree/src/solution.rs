use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end_word: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        let word_chars: Vec<char> = word.chars().collect();
        for letter in word_chars {
            if node.children.get(&letter).is_some() {
                node = node.children.get_mut(&letter).unwrap();
            } else {
                node = node.children.entry(letter).or_insert(TrieNode::new());
            }
        }
        node.end_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let search_word: Vec<char> = word.chars().collect();
        let mut node = &self.root;

        for letter in search_word {
            if node.children.get(&letter).is_some() {
                node = node.children.get(&letter).unwrap();
            } else {
                return false;
            }
        }
        return node.end_word;
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let search_prefix: Vec<char> = prefix.chars().collect();
        let mut node = &self.root;

        for letter in search_prefix {
            if node.children.get(&letter).is_some() {
                node = node.children.get(&letter).unwrap();
            } else {
                return false;
            }
        }
        return true;
    }
}
