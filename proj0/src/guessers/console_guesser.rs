use std::io;

use super::Guesser;

pub type ConsoleGuesser = ();

impl Guesser for ConsoleGuesser {
    fn new(_possible_words: Vec<String>) -> Self {}

    /// Reads a guess in from stdin
    fn guess(&self, _pattern: &str, _guesses: &[char]) -> char {
        let default = '🧀';
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_err() {
            return default;
        }
        buffer.trim().parse().unwrap_or(default)
    }
}
