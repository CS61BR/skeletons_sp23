use super::Chooser;
use crate::random::Random;

pub struct RandomChooser {
    word: String,
    pattern: String,
}

impl Chooser for RandomChooser {
    fn new(possible_words: Vec<String>, mut random: Random) -> Self {
        unimplemented!() // TODO: fill in this method
    }

    fn make_guess(&mut self, guess: char) -> usize {
        unimplemented!() // TODO: fill in this method
    }

    fn get_pattern(&self) -> &str {
        unimplemented!() // TODO: fill in this method
    }

    fn get_word(&self) -> &str {
        unimplemented!() // TODO: fill in this method
    }
}
