use std::num::Wrapping;

use crate::bomb_answers::{password0, password1, password2};

#[test]
fn check_bomb_password0() {
    assert_eq!(scramble(&password0()), 2748841116325181568);
}

#[test]
fn check_bomb_password1() {
    assert_eq!(scramble(&password1().to_string()), 8998170376785546403);
}

#[test]
fn check_bomb_password2() {
    let attempt = password2();
    let piece = &attempt.split(' ').nth(1337).unwrap();
    assert_eq!(scramble(piece), 17170708448383273287);
}

fn scramble(s: &str) -> u64 {
    let mut r = Wrapping::<u64>(654321);
    for b in s.as_bytes() {
        r = (r + Wrapping(u64::from(*b))) * Wrapping(16088033396387240377);
        r = (r << 19) | (r >> 45);
    }
    r.0
}
