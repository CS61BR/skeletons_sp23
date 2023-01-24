use std::io::{BufRead, Error, Write};

use crate::prompt_choice;

pub fn play_species_list_stage<R, W>(input: &mut R, output: &mut W) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    writeln!(
        output,
        "Inside Professor Hug's office, you see some O'Reilly books. These \
        books have cool animals on the covers. As a budding computer \
        scientist, you should be able to identify all kinds of neat animals. \
        Here's a few:"
    )?;

    const PROMPTS_1: [&str; 2] = [
        "These large felines with spots will teach you how to react quickly.",
        "This native american bovine can be found in the plains, and happens \
        to be EXTREMELY good at Java.",
    ];
    const ANSWERS_1: [&str; 2] = ["leopards", "bison"];
    const PROMPTS_2: [&str; 2] = [
        "These bushy-tailed friends are everywhere in and around the trees \
        on campus, and know the best parts of Java.",
        "These tiny birds flap very fast, drink nectar, and know how to make \
        simpler Java applications.",
    ];
    const ANSWERS_2: [&str; 2] = ["squirrels", "hummingbirds"];

    print_prompts(output, &PROMPTS_1)?;
    handle_responses(input, output, &ANSWERS_1)?;
    writeln!(output, "Woah! There are even more neat books here!")?;
    print_prompts(output, &PROMPTS_2)?;
    handle_responses(input, output, &ANSWERS_2)?;
    writeln!(
        output,
        "Well, there's nothing left here! press enter to move."
    )?;
    handle_responses(input, output, &[])?;

    writeln!(
        output,
        "Wow! That was pretty neat! We got to see so many neat animals! We \
        should study now, so let's go to the Woz."
    )?;
    prompt_choice(input, output, &["go"])?;
    Ok(())
}

fn print_prompts<W: Write>(output: &mut W, prompts: &[&str]) -> Result<(), Error> {
    for p in prompts {
        writeln!(output, "- {}", p)?;
    }
    writeln!(
        output,
        "Type their names into the terminal (separated by ',')"
    )
}

fn handle_responses<R, W>(input: &mut R, output: &mut W, answers: &[&str]) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    loop {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        let lower = buf.to_lowercase();
        let responses = lower.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        let similarity = similarity(&responses, answers);
        if similarity == 1.0 {
            break Ok(());
        }
        writeln!(
            output,
            "Try again! You got {} animals correct!",
            (similarity * answers.len() as f64).round() as usize
        )?;
    }
}

fn similarity(one: &[&str], two: &[&str]) -> f64 {
    let count = two.iter().filter(|s| one.contains(s)).count();
    (count / two.len()) as f64
}
