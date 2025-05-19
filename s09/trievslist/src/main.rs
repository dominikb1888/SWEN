use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_word: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::default()),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current
                .children
                .entry(ch)
                .or_insert_with(|| Box::new(TrieNode::default()));
        }
        current.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(next) => current = next,
                None => return false,
            }
        }
        current.is_word
    }
}

fn save_to_file<T: serde::Serialize>(data: &T, path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let encoded = bincode::serialize(data).expect("Failed to serialize");
    writer.write_all(&encoded)?;
    Ok(())
}

fn file_size(path: &str) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

fn main() -> std::io::Result<()> {
    let wordlist = vec![
        "cat", "car", "cart", "dog", "dot", "dove", "door", "apple", "app", "apt", "banana",
    ];

    // Save word list
    save_to_file(&wordlist, "wordlist.bin")?;

    // Build and save trie
    let mut trie = Trie::new();
    for word in &wordlist {
        trie.insert(word);
    }
    save_to_file(&trie, "trie.bin")?;

    // Compare sizes
    let list_size = file_size("wordlist.bin");
    let trie_size = file_size("trie.bin");

    println!("wordlist.bin size: {} bytes", list_size);
    println!("trie.bin size:     {} bytes", trie_size);

    Ok(())
}

