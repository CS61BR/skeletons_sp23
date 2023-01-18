use std::{
    fs,
    io::{self, BufRead, Write},
    time::Instant,
};

use choosers::{random_chooser::RandomChooser, Chooser};
use guessers::{console_guesser::ConsoleGuesser, Guesser};

use crate::random::Random;

mod choosers;
mod guessers;
mod random;
#[cfg(test)]
mod tests;

#[allow(dead_code)] // allow this constant to go unused
const LARGE_DICTIONARY_FILE: &str = "data/sorted_scrabble.txt";
#[allow(dead_code)] // allow this constant to go unused
const SMALL_DICTIONARY_FILE: &str = "data/example.txt";

fn main() -> io::Result<()> {
    play_game::<RandomChooser, ConsoleGuesser>()
}

fn play_game<C: Chooser, G: Guesser>() -> io::Result<()> {
    println!("Welcome to Awakening of Azozoth!");
    println!();
    let word_len = prompt("What length word do you want to use? ")?;
    let max_guesses = prompt("How many wrong answers allowed? ")?;
    println!();

    // new Random, seeded with the current time
    let random = Random::new(&format!("{:?}", Instant::now()));

    // read in words from the dictionary file
    let words = read_from_file(LARGE_DICTIONARY_FILE, word_len)?;

    // initialize both "players" and game variables
    let mut chooser = C::new(words.clone(), random);
    let guesser = G::new(words);
    let mut guesses = Vec::new();
    let mut guesses_remaining = max_guesses;

    // main game loop
    while guesses_remaining > 0 && chooser.get_pattern().contains('-') {
        println!("Guesses: {guesses_remaining}");
        println!("Guessed: {:?}", guesses);
        println!("Current: {}", chooser.get_pattern());
        let (guess, revealed_count) = loop {
            print!("Your guess? ");
            io::stdout().flush()?;
            let g = guesser.guess(chooser.get_pattern(), &guesses);
            if ('a'..='z').contains(&g) && !guesses.contains(&g) {
                break (g, chooser.make_guess(g));
            }
            println!("Invalid guess.");
        };
        guesses.push(guess);
        match revealed_count {
            0 => {
                println!("Sorry, there are no '{guess}'s.");
                guesses_remaining -= 1;
            }
            1 => println!("Yes, there is one '{guess}'."),
            n => println!("Yes, there are {n} '{guess}'s."),
        }
        println!();
    }

    // game is over, report results
    let word = chooser.get_word();
    if chooser.get_pattern().contains('-') {
        println!("Azozoth awakens. You lose!");
        println!("My word was '{word}'.");
    } else {
        println!("'{word}' was my word! Azozoth resumes its slumber.");
    }
    Ok(())
}

fn prompt(message: &str) -> io::Result<usize> {
    loop {
        print!("{}", message);
        io::stdout().flush()?;
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim().parse() {
            Ok(x) => return Ok(x),
            Err(_) => println!("Not a nonnegative integer!"),
        }
    }
}

fn read_from_file(path: &str, word_len: usize) -> io::Result<Vec<String>> {
    let words_file = fs::File::open(path)?;
    Ok(io::BufReader::new(words_file)
        .lines()
        .filter_map(|r| match r {
            Ok(s) if s.len() == word_len => Some(s),
            _ => None,
        })
        .collect::<Vec<String>>())
}
