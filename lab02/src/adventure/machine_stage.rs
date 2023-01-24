use std::io::{BufRead, Error, Write};

use crate::prompt_choice;

pub fn play_machine_stage<R, W>(input: &mut R, output: &mut W) -> Result<(), Error>
where
    R: BufRead,
    W: Write,
{
    writeln!(
        output,
        "On the first (zeroth?) floor of Soda, below the labs, you find a \
        mysterious machine. It has holes for two lists of ints of the same \
        length, and a third hole that looks like it would output a number. \
        The label reads:\
        \n\n'SumOfElementWiseMax-inator'\n\n\
        ... Huh. You decide to experiment with the machine for a bit."
    )?;

    loop {
        let prompt_1 = "Enter a sequence of ints, separated by commas:";
        let list_1 = prompt_for_list(input, output, prompt_1)?;
        let prompt_2 = "Enter a second sequence, with the same number of ints:";
        let list_2 = prompt_for_list(input, output, prompt_2)?;
        if list_1.len() != list_2.len() {
            writeln!(output, "The sequences don't have the same length!")?;
            continue;
        }

        let machine_result = sum_of_elementwise_maxes(&list_1, &list_2);
        writeln!(
            output,
            "The machine whirrs briefly before outputting a slip of paper, \
            reading {machine_result}"
        )?;
        writeln!(output, "Do you want to continue experimenting?")?;
        let c = prompt_choice(input, output, &["experiment", "move on"])?;
        if c == 1 {
            break;
        }
    }

    writeln!(
        output,
        "It's almost time for lecture! How are you attending?"
    )?;
    let c = prompt_choice(input, output, &["li ka shing", "zoom"])?;
    let story = match c {
        0 => "You head to Li Ka Shing 245, and sit in your favorite seat.",
        _ => "You find an open seat, set up your laptop, and join the Zoom.",
    };
    writeln!(output, "{}", story)
}

fn prompt_for_list<R, W>(input: &mut R, output: &mut W, prompt: &str) -> Result<Vec<i32>, Error>
where
    R: BufRead,
    W: Write,
{
    loop {
        writeln!(output, "{prompt}")?;
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        if let Ok(v) = buf.split(',').map(|s| s.trim().parse::<i32>()).collect() {
            return Ok(v);
        }
        writeln!(
            output,
            "Hmm, are you sure you typed a sequence of integers?"
        )?;
    }
}

fn sum_of_elementwise_maxes(a: &[i32], b: &[i32]) -> i32 {
    array_sum(&array_max(a, b))
}

fn array_max(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut return_vec = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        let bigger_value = max(a[i], b[i]);
        return_vec.push(bigger_value);
    }
    return_vec
}

fn array_sum(v: &Vec<i32>) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    while i < v.len() {
        sum = sum + add(sum, v[i]);
        i += 1;
    }
    sum
}

fn max(a: i32, b: i32) -> i32 {
    // If you're stepping into this function, just step out
    // because you're not going to learn anything
    // If there is a bug here, just rewrite this function from scratch
    let w = (b - a) >> 31;
    let z = !(b - a) >> 31;
    b & w | a & z
}

fn add(a: i32, b: i32) -> i32 {
    // If you're stepping into this function, just step out
    // because you're not going to learn anything
    // If there is a bug here, just rewrite this function from scratch
    let x = a;
    let y = b;
    let mut and = x & y;
    let mut xor = x ^ y;
    while and != 0 {
        and <<= 1;
        let temp = xor ^ and;
        and &= xor;
        xor = temp;
    }
    xor
}
