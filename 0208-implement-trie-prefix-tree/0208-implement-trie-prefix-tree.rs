use std::collections::{ HashMap };


struct TrieNode {
    is_eow: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_eow: false,
            children: HashMap::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut cur_node = &mut self.root;

        for word_item in word.chars() {
            cur_node = cur_node.children.entry(word_item).or_insert(TrieNode::new());
        }
        cur_node.is_eow = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut cur_node = &self.root;

        for word_item in word.chars() {
            match cur_node.children.get(&word_item) {
                Some(child_node) => cur_node = child_node,
                None => return false,
            }
        }
        cur_node.is_eow
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur_node = &self.root;

        for word_item in prefix.chars() {
            match cur_node.children.get(&word_item) {
                Some(child_node) => cur_node = child_node,
                None => return false,
            }
        }

        if cur_node.is_eow {
            return true;
        }
        cur_node.children.len() != 0
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */