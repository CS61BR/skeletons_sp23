use std::{
    io::{BufRead, BufReader, Error, Write},
    str::FromStr,
};

use bee_counting_stage::play_bee_counting_stage;
use machine_stage::play_machine_stage;
use palindrome_stage::play_palindrome_stage;
use species_list_stage::play_species_list_stage;

mod bee_counting_stage;
mod machine_stage;
mod palindrome_stage;
mod species_list_stage;
#[cfg(test)]
mod tests;

fn main() -> Result<(), Error> {
    let mut input = BufReader::new(std::io::stdin());
    let mut output = std::io::stdout();

    writeln!(
        output,
        "It's a wonderful day of learning about computer science! We are going \
        to see so many cool things today! Let's go! (To answer the prompts, \
        type the possibilities in the brackets to go to that choice)"
    )?;
    prompt_choice(&mut input, &mut output, &["go"])?;

    play_bee_counting_stage(&mut input, &mut output)?;
    play_species_list_stage(&mut input, &mut output)?;
    play_palindrome_stage(&mut input, &mut output)?;
    play_machine_stage(&mut input, &mut output)?;

    writeln!(
        output,
        "All in another fun day of learning computer science :)"
    )
}

fn prompt_choice<R, W>(input: &mut R, output: &mut W, choices: &[&str]) -> Result<usize, Error>
where
    R: BufRead,
    W: Write,
{
    write!(output, ">>")?;
    for c in choices {
        write!(output, " [{}]", c)?;
    }
    writeln!(output)?;
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        if let Some(n) = parse_choice(buf.trim(), choices) {
            return Ok(n);
        }
        writeln!(
            output,
            "Sorry, I didn't understand that. Please type one of the responses \
            in the brackets!"
        )?;
    }
}

fn prompt_number<R, W, F>(input: &mut R, output: &mut W) -> Result<F, Error>
where
    R: BufRead,
    W: Write,
    F: FromStr,
{
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        if let Ok(n) = buf.trim().parse() {
            return Ok(n);
        }
        writeln!(
            output,
            "Sorry, I didn't understand that. Please type a number!"
        )?;
    }
}

fn parse_choice(input: &str, choices: &[&str]) -> Option<usize> {
    if input.is_empty() {
        return None;
    }
    for (i, &c) in choices.iter().enumerate() {
        if c.to_lowercase() == input.to_lowercase() {
            return Some(i);
        }
    }
    for (i, &c) in choices.iter().enumerate() {
        if c.to_lowercase().contains(&input.to_lowercase()) {
            return Some(i);
        }
    }
    None
}
