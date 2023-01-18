use std::collections::BTreeMap;

pub mod console_guesser;
pub mod letter_freq_guesser;
pub mod paga_lfg;
pub mod pattern_aware_lfg;

pub trait Guesser {
    fn new(possible_words: Vec<String>) -> Self;
    fn guess(&self, pattern: &str, guesses: &[char]) -> char;
}

fn freq_map(words: &[String]) -> BTreeMap<char, usize> {
    unimplemented!();
    // TODO: implement this function
    // you may change the function signature if you like
}
