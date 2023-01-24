use std::io::{BufRead, Error, Write};

use crate::{prompt_choice, prompt_number};

pub fn play_bee_counting_stage<R, W>(input: &mut R, output: &mut W) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    writeln!(
        output,
        "In Soda 326, you can find the computers known as \"The Hive\". It is \
        a little-known fact that they are called this because they are home to \
        (friendly) robotic bees. How many bees do you see?"
    )?;

    const NUM_ROUNDS: usize = 3;
    const ROUNDS: [usize; NUM_ROUNDS] = [3, 2, 6];
    let mut responses = [0; NUM_ROUNDS];

    loop {
        for i in 0..NUM_ROUNDS {
            if i > 0 {
                writeln!(output, "How about now?")?;
            }
            writeln!(output, "{}", " -.-".repeat(ROUNDS[i]))?;
            responses[0] = prompt_number(input, output)?;
        }

        if responses == ROUNDS {
            break;
        }
        writeln!(
            output,
            "You did not count the bees correctly. Let's try again!"
        )?;
    }

    writeln!(output, "Those sure were some bees!")?;
    writeln!(
        output,
        "Phew, that was a lot of counting! It's time for Professor Hug's \
        office hours! Let's head up to his office on the 7th floor."
    )?;
    prompt_choice(input, output, &["go"])?;
    Ok(())
}
