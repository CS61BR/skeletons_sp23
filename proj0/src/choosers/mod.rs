use crate::random::Random;

pub mod cheese_chooser;
pub mod evil_chooser;
pub mod random_chooser;

pub trait Chooser {
    fn new(possible_words: Vec<String>, random: Random) -> Self;
    fn make_guess(&mut self, guess: char) -> usize;
    fn get_pattern(&self) -> &str;
    fn get_word(&self) -> &str;
}
