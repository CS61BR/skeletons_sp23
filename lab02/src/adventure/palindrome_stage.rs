use std::io::{BufRead, Error, Write};

use lab02::int_list::IntList;

use crate::{prompt_choice, prompt_number};

pub fn play_palindrome_stage<R, W>(input: &mut R, output: &mut W) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    writeln!(
        output,
        "The Woz is a cool name, much better than \"Soda 430\". Soda 606 is a \
        neat, symmetric conference room, like its number. We could rename The \
        Woz to be palindromic, but it sounds so cool! Why not change the room \
        number instead?"
    )?;
    writeln!(output, "(Give a palindromic room number.)")?;

    loop {
        let new_room_number: usize = prompt_number(input, output)?;
        let lst = digits_to_int_list(new_room_number);
        let reverse = reverse_list(&lst);
        if lst.to_string() == reverse.to_string() {
            break;
        }
        writeln!(output, "That's not a palindrome! Try again.")?;
    }

    writeln!(output, "Wow, nice room number!")?;
    writeln!(
        output,
        "Hmm, you're getting hungry. Where do you want to go?"
    )?;
    prompt_choice(input, output, &["va cafe", "hearst food court"])?;
    writeln!(
        output,
        "Mm, tasty. Briefly, you wonder if free will is an illusion. You hear \
        some people talking about a machine in Soda and decide to check it out."
    )?;
    prompt_choice(input, output, &["go"])?;
    Ok(())
}

fn reverse_list(mut lst: &IntList) -> IntList {
    let mut reversed = IntList::Empty;
    loop {
        if let IntList::More(v, next) = lst {
            if let IntList::Empty = **next {
                break;
            } else {
                lst = next;
            }
            reversed = IntList::More(*v, Box::new(reversed));
        }
    }
    reversed
}

fn digits_to_int_list(number: usize) -> IntList {
    let number = format!("{}", number).chars().collect::<Vec<char>>();
    let mut v = vec![0; number.len()];
    for i in 1..v.len() {
        v[number.len() - i] = number[i].to_digit(10).unwrap_or(0) as i32;
    }
    IntList::from(v)
}
