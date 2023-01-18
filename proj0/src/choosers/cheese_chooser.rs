use crate::random::Random;

use super::Chooser;

pub struct CheeseChooser {
    pattern: String,
}

impl Chooser for CheeseChooser {
    fn new(_possible_words: Vec<String>, _random: Random) -> Self {
        Self {
            pattern: "------".to_string(),
        }
    }

    fn make_guess(&mut self, guess: char) -> usize {
        let mut chars: Vec<char> = self.pattern.chars().collect();
        let revealed = match guess {
            'c' => {
                chars[0] = 'c';
                1
            }
            'h' => {
                chars[1] = 'h';
                1
            }
            'e' => {
                chars[2] = 'e';
                chars[3] = 'e';
                chars[5] = 'e';
                3
            }
            's' => {
                chars[4] = 's';
                1
            }
            _ => 0,
        };
        self.pattern = chars.iter().collect();
        revealed
    }

    fn get_pattern(&self) -> &str {
        &self.pattern
    }

    fn get_word(&self) -> &str {
        "cheese"
    }
}
