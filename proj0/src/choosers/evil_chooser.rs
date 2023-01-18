use super::Chooser;


pub struct EvilChooser {
    possible_words: Vec<String>,
    pattern: String
}

impl Chooser for EvilChooser {
    fn new(possible_words: Vec<String>, _random: crate::random::Random) -> Self {
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
