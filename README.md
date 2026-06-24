# leven-automaton

A Levenshtein automaton implementation in Rust. Given a pattern string and a maximum edit distance, it pre-computes a deterministic finite automaton (DFA) that can efficiently test whether any input word is within the allowed Levenshtein distance from the pattern.

## What is a Levenshtein Automaton?

A Levenshtein automaton accepts all strings that are within a given edit distance *k* of a fixed pattern string. The three edit operations counted are:

- **Substitution** - replacing one character with another
- **Insertion** - adding a character
- **Deletion** - removing a character

The automaton is built once for a given pattern and distance threshold, then can match many words in O(n) time per word (where n is the word length), with no additional allocations.

## Usage

```rust
use leven_automaton::automaton::LevenshteinAutomaton;

fn main() {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let automaton = LevenshteinAutomaton::new("hello", 2, alphabet)
        .expect("failed to build automaton");

    assert!(automaton.match_word("hello"));  // exact match (distance 0)
    assert!(automaton.match_word("hallo"));  // 1 substitution
    assert!(automaton.match_word("helo"));   // 1 deletion
    assert!(automaton.match_word("helloo")); // 1 insertion
    assert!(!automaton.match_word("world")); // distance 4, exceeds threshold
}
```

## API

### `LevenshteinAutomaton::new(pattern, diffs_allowed, alphabet) -> Option<Self>`

Builds the automaton. Returns `None` if the state space overflows (very long patterns with large edit distances).

- `pattern` - the target string to match against
- `diffs_allowed` - maximum number of edits permitted
- `alphabet` - the set of characters the automaton will recognize. Words containing characters outside this alphabet will not match.

### `automaton.match_word(word) -> bool`

Returns `true` if the Levenshtein distance between `word` and the pattern is at most `diffs_allowed`.

## How It Works

1. **State representation**: Each state is a vector of edit distances between prefixes of the pattern and the input consumed so far. Values are clamped to `diffs_allowed + 1` to bound the state space.

2. **State identification**: States are encoded as unique integer IDs using a mixed-radix representation (base `diffs_allowed + 2`), enabling efficient deduplication via a HashMap.

3. **Automaton construction**: A BFS explores all reachable states from the initial state, computing transitions for each character in the alphabet. Equivalent states (same edit distance vector) are merged.

4. **Matching**: The input word is consumed character by character, following transitions in the pre-computed DFA. The final state determines acceptance.

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

The test suite covers:
- State initialization and edit distance computation
- State ID generation and uniqueness
- Character transitions (match, substitution, insertion, deletion)
- Acceptance criteria
- End-to-end matching with exact matches, edits, edge cases (empty strings, single characters), and alphabet restrictions

## Project Structure

```
src/
  main.rs              - Entry point and module declarations
  state/mod.rs         - State and StateId types, edit distance logic
  automaton/mod.rs     - LevenshteinAutomaton, BFS construction, matching
```

## License

MIT
