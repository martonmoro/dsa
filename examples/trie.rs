// Implement a trie (prefix tree) with insert, search, and starts_with.
//   - insert(word):       add a word into the trie
//   - search(word):       return true if the exact word is in the trie
//   - starts_with(prefix): return true if any word in the trie starts with prefix
 
use std::collections::HashMap;
 
#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool, // marks the end of a complete word
}
 
struct Trie {
    root: TrieNode,
}
 
impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::default() }
    }
 
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            // entry().or_default() creates the child node if it's missing
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
    }
 
    fn search(&self, word: String) -> bool {
        match self.walk(&word) {
            Some(node) => node.is_end, // word exists only if we land on a word-end
            None => false,
        }
    }
 
    fn starts_with(&self, prefix: String) -> bool {
        self.walk(&prefix).is_some() // reaching the end of the prefix is enough
    }
 
    // Follow the characters from the root; return None if any step is missing.
    fn walk(&self, s: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for c in s.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }
}
 
fn main() {
    let mut trie = Trie::new();
 
    trie.insert("apple".to_string());
 
    println!("search('apple')       = {}", trie.search("apple".to_string())); // true
    println!("search('app')         = {}", trie.search("app".to_string())); // false
    println!("starts_with('app')    = {}", trie.starts_with("app".to_string())); // true
 
    trie.insert("app".to_string());
    println!("search('app')         = {}", trie.search("app".to_string())); // true
 
    println!("search('apricot')     = {}", trie.search("apricot".to_string())); // false
    println!("starts_with('ap')     = {}", trie.starts_with("ap".to_string())); // true
    println!("starts_with('b')      = {}", trie.starts_with("b".to_string())); // false
 
    assert!(trie.search("apple".to_string()));
    assert!(trie.search("app".to_string()));
    assert!(!trie.search("appl".to_string()));
    assert!(trie.starts_with("appl".to_string()));
    assert!(!trie.starts_with("xyz".to_string()));
 
    println!("All test cases passed ✅");
}
 
