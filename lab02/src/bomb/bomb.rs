use std::num::Wrapping;

use lab02::{int_list::IntList, random::Random};

use crate::bomb_answers::{password0, password1, password2};

mod bomb_answers;
#[cfg(test)]
mod tests;

pub fn main() {
    // phase 0
    let correct_password = shuffle("I like cheese");
    let attempt = password0();
    if attempt != correct_password {
        panic!("Phase 0 went BOOM!");
    }
    println!("You passed Phase 0 with the password {}", attempt);

    // phase 1
    let correct_password = shuffle_list("cheese is good");
    let attempt = password1();
    if attempt.to_string() != correct_password.to_string() {
        panic!("Phase 1 went BOOM!");
    }
    println!("You passed Phase 1 with the password {}", attempt);

    // phase 2
    let attempt = password2();
    let pieces = attempt.split(' ').collect::<Vec<&str>>();
    let mut rand = Random::new("quantity has its own quality");
    let mut correct = false;
    for (i, n) in (0..10000).map(|_| rand.next_u64()).enumerate() {
        if i == 1337 && pieces[i] == n.to_string() {
            correct = true;
        }
    }
    if !correct {
        panic!("Phase 2 went BOOM!");
    }
    println!("You passed Phase 2 with the password {}", attempt);
}

fn shuffle(s: &str) -> String {
    let mut r = Wrapping::<u64>(123456);
    for b in s.as_bytes() {
        r = (r + Wrapping(u64::from(*b))) * Wrapping(11400714819323198485);
        r = (r << 25) | (r >> 39);
    }
    format!("{}", r)
}

fn shuffle_list(s: &str) -> IntList {
    let r = shuffle(s);
    let mut lst = IntList::Empty;
    for b in r.as_bytes() {
        if (b'0'..=b'9').contains(b) {
            lst = IntList::More(i32::from(b - b'0'), Box::new(lst));
        }
    }
    lst
}
