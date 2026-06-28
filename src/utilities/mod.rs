
use std::{collections::HashMap};

#[derive(Clone)]
pub struct TrieNode{
    nodes: HashMap<char,TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self{
        TrieNode {
            nodes : HashMap::new(),
            word : None,
        }
    }

    pub fn add_new_word(& mut self, word : &str) -> bool {
        let mut curr_node = self;
        
        for c in word.chars() {
            let temp = curr_node.nodes.entry(c).or_insert(TrieNode { nodes: HashMap::new(), word: None });
            curr_node = temp;
        }

        curr_node.word = Some(word.to_owned());  
        true 
     }

     pub fn does_word_exist(&self, word : &str)-> bool{
        self.does_word_exist_internal(word).unwrap_or(false)
     }

    fn does_word_exist_internal(&self, word : &str)-> Option<bool> {
        let mut node = self;
        
        for c in word.chars() {
            node = node.nodes.get(&c)?;
        }

        node.word.as_ref().map(|s| *s == word)
    }

    pub fn get_next_node(&self, c : char) -> Option<&TrieNode>{
        self.nodes.get(&c)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_trie() {
        let trie = TrieNode::new();
        assert!(trie.nodes.is_empty());
        assert!(trie.word.is_none());
    }

    #[test]
    fn add_single_word() {
        let mut trie = TrieNode::new();
        assert!(trie.add_new_word("hello"));
        assert!(trie.does_word_exist("hello"));
    }

    #[test]
    fn search_nonexistent_word() {
        let mut trie = TrieNode::new();
        trie.add_new_word("hello");
        assert!(!trie.does_word_exist("world"));
    }

    #[test]
    fn search_empty_trie() {
        let trie = TrieNode::new();
        assert!(!trie.does_word_exist("hello"));
    }

    #[test]
    fn prefix_is_not_a_word() {
        let mut trie = TrieNode::new();
        trie.add_new_word("hello");
        assert!(!trie.does_word_exist("hel"));
    }

    #[test]
    fn word_is_not_found_by_extension() {
        let mut trie = TrieNode::new();
        trie.add_new_word("hel");
        assert!(!trie.does_word_exist("hello"));
    }

    #[test]
    fn add_multiple_words() {
        let mut trie = TrieNode::new();
        trie.add_new_word("cat");
        trie.add_new_word("car");
        trie.add_new_word("card");
        assert!(trie.does_word_exist("cat"));
        assert!(trie.does_word_exist("car"));
        assert!(trie.does_word_exist("card"));
        assert!(!trie.does_word_exist("ca"));
    }

    #[test]
    fn add_word_that_is_prefix_of_existing() {
        let mut trie = TrieNode::new();
        trie.add_new_word("hello");
        trie.add_new_word("hel");
        assert!(trie.does_word_exist("hello"));
        assert!(trie.does_word_exist("hel"));
    }

    #[test]
    fn add_word_that_extends_existing() {
        let mut trie = TrieNode::new();
        trie.add_new_word("hel");
        trie.add_new_word("hello");
        assert!(trie.does_word_exist("hel"));
        assert!(trie.does_word_exist("hello"));
    }

    #[test]
    fn add_duplicate_word() {
        let mut trie = TrieNode::new();
        trie.add_new_word("hello");
        trie.add_new_word("hello");
        assert!(trie.does_word_exist("hello"));
    }

    #[test]
    fn single_char_word() {
        let mut trie = TrieNode::new();
        trie.add_new_word("a");
        assert!(trie.does_word_exist("a"));
        assert!(!trie.does_word_exist("b"));
    }

    #[test]
    fn empty_word_supported() {
        let mut trie = TrieNode::new();
        trie.add_new_word("");
        assert!(trie.does_word_exist(""));
    }

    #[test]
    fn empty_word_not_added() {
        let trie = TrieNode::new();
        assert!(!trie.does_word_exist(""));
    }

    #[test]
    fn words_with_shared_prefix() {
        let mut trie = TrieNode::new();
        trie.add_new_word("abc");
        trie.add_new_word("abd");
        trie.add_new_word("xyz");
        assert!(trie.does_word_exist("abc"));
        assert!(trie.does_word_exist("abd"));
        assert!(trie.does_word_exist("xyz"));
        assert!(!trie.does_word_exist("ab"));
        assert!(!trie.does_word_exist("xy"));
    }

    #[test]
    fn get_next_node_existing_char() {
        let mut trie = TrieNode::new();
        trie.add_new_word("abc");
        let node = trie.get_next_node('a');
        assert!(node.is_some());
        let node = node.unwrap();
        assert!(node.get_next_node('b').is_some());
    }

    #[test]
    fn get_next_node_missing_char() {
        let mut trie = TrieNode::new();
        trie.add_new_word("abc");
        assert!(trie.get_next_node('x').is_none());
    }

    #[test]
    fn get_next_node_walk_full_word() {
        let mut trie = TrieNode::new();
        trie.add_new_word("cat");
        let c = trie.get_next_node('c').unwrap();
        let a = c.get_next_node('a').unwrap();
        let t = a.get_next_node('t').unwrap();
        assert_eq!(t.word, Some("cat".to_string()));
    }

    #[test]
    fn get_next_node_intermediate_has_no_word() {
        let mut trie = TrieNode::new();
        trie.add_new_word("cat");
        let c = trie.get_next_node('c').unwrap();
        assert!(c.word.is_none());
    }

    #[test]
    fn many_words() {
        let mut trie = TrieNode::new();
        let words = vec!["apple", "app", "application", "bat", "ball", "band", "banana"];
        for w in &words {
            trie.add_new_word(w);
        }
        for w in &words {
            assert!(trie.does_word_exist(w));
        }
        assert!(!trie.does_word_exist("ap"));
        assert!(!trie.does_word_exist("ba"));
        assert!(!trie.does_word_exist("ban"));
        assert!(!trie.does_word_exist("banan"));
    }
}
