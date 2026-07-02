use std::env;
use std::fs;
use std::io;

use crate::matcher::Matcher;

/// Module containing the State and StateId types for representing automaton states.
pub mod state;

/// Module containing the LevenshteinAutomaton type for building and querying the DFA.
pub mod automaton;
/// Module containing the Matcher type for fuzzy matching against a dictionary.
mod matcher;

/// Module containing the Trie data structure for efficient word storage and traversal.
pub mod utilities;

fn main() {
    let path = env::args().nth(1).expect("Usage: levenshtein-automaton <dictionary>");
    let contents = fs::read_to_string(&path).expect("Failed to read dictionary");
    let dictionary : Vec<String> = contents.lines().map(String::from).collect();
    let matcher = Matcher::new(&dictionary).unwrap();
    
    loop{
        let mut pattern = String::new();
        let mut diffs_allowed = String::new();
        println!("Enter the pattern");
        io::stdin().read_line(&mut pattern).expect("Failed to read input");
        println!("Enter the number of diffs");
        io::stdin().read_line(&mut diffs_allowed).expect("Failed to diffs allowed");
        let pattern = pattern.trim();
        let diffs = diffs_allowed.trim().parse().expect("Not a valid number");
        let result = matcher.match_pattern(pattern, diffs);

        let Some(v) = result else{
            println!("Couldn't find any match");
            continue;
        };

        println!("{:#?}",v);
    }
}