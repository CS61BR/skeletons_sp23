use crate::{
    guessers::{pattern_aware_lfg::PatternAwareLFG, Guesser},
    read_from_file, LARGE_DICTIONARY_FILE,
};

fn check_guess_order(pattern: &str, words: &[String], correct_order: &str) {
    let guesser = PatternAwareLFG::new(Vec::from(words));
    let mut guesses = Vec::new();
    for a in correct_order.chars() {
        assert_eq!(guesser.guess(&pattern, &guesses), a);
        guesses.push(a);
    }
}

#[test]
fn foods() {
    check_guess_order("------", &["cheese".to_owned()], "echs");
    check_guess_order("---------", &["blueberry".to_owned()], "berluy");
    check_guess_order("------", &["potato".to_owned()], "otap");
}

#[test]
fn simple_patterns() {
    let words = [
        "aaaar", "bbbbs", "cccct", "ddddu", "eeeev", "ffffx", "ffffw",
    ]
    .map(|s| s.to_owned());
    check_guess_order("----r", &words, "ar");
    check_guess_order("----s", &words, "bs");
    check_guess_order("b----", &words, "bs");
    check_guess_order("c-c--", &words, "ct");
    check_guess_order("-d--u", &words, "du");
    check_guess_order("ffff-", &words, "fwx");
}

#[test]
fn large() {
    let words = read_from_file(LARGE_DICTIONARY_FILE, 6).unwrap();
    check_guess_order("--a--y", &words, "yalrstghnebcpmwdfkouivqz");
    check_guess_order("-e-e--", &words, "erstdlnbacvmphwfgykoijuxz");
    check_guess_order("-h---n", &words, "nhoatseiprcwykzglmuv");
    check_guess_order("---e-s", &words, "esrlatoinpcmdbuwvgyhkfzxjq");
    check_guess_order("c--e--", &words, "ecsroatnylihudmvpbkfzgw");
    check_guess_order("----ed", &words, "edalortisnpugbcmhkfwvyzjxq");
    check_guess_order("un----", &words, "nuesialodrtcpkwhgmbfyvxjqz");
}
