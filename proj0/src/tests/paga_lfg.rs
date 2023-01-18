use crate::{
    guessers::{paga_lfg::PagaLFG, Guesser},
    read_from_file, LARGE_DICTIONARY_FILE,
};

fn from_large_file(word_len: usize) -> PagaLFG {
    PagaLFG::new(read_from_file(LARGE_DICTIONARY_FILE, word_len).unwrap())
}

fn check_guess(guesser: &PagaLFG, pattern: &str, previous_guesses: &str, correct_guess: char) {
    let chars = previous_guesses.chars().collect::<Vec<char>>();
    assert_eq!(guesser.guess(pattern, &chars), correct_guess);
}

#[test]
fn trace_cheese() {
    let guesser = from_large_file(6);
    check_guess(&guesser, "------", "", 'e');
    check_guess(&guesser, "--ee-e", "e", 's');
    check_guess(&guesser, "--eese", "es", 'c');
    check_guess(&guesser, "c-eese", "esc", 'h');
}

#[test]
fn trace_untrusty() {
    let guesser = from_large_file(8);
    check_guess(&guesser, "--------", "", 'e');
    check_guess(&guesser, "--------", "e", 'i');
    check_guess(&guesser, "--------", "ei", 'a');
    check_guess(&guesser, "--------", "eia", 'o');
    check_guess(&guesser, "--------", "eiao", 'u');
    check_guess(&guesser, "u---u---", "eiaou", 'c');
    check_guess(&guesser, "u---u---", "eiaouc", 't');
    check_guess(&guesser, "u-t-u-t-", "eiaouct", 'n');
    check_guess(&guesser, "unt-u-t-", "eiaouctn", 'r');
    check_guess(&guesser, "untru-t-", "eiaouctnr", 's');
    check_guess(&guesser, "untrust-", "eiaouctnrs", 'y');
}

#[test]
fn trace_lugworms() {
    let guesser = from_large_file(8);
    check_guess(&guesser, "--------", "", 'e');
    check_guess(&guesser, "--------", "e", 'i');
    check_guess(&guesser, "--------", "ei", 'a');
    check_guess(&guesser, "--------", "eia", 'o');
    check_guess(&guesser, "----o---", "eiao", 'u');
    check_guess(&guesser, "-u--o---", "eiaou", 's');
    check_guess(&guesser, "-u--o--s", "eiaous", 'c');
    check_guess(&guesser, "-u--o--s", "eiaousc", 'r');
    check_guess(&guesser, "-u--or-s", "eiaouscr", 'g');
    check_guess(&guesser, "-ug-or-s", "eiaouscrg", 'm');
    check_guess(&guesser, "-ug-orms", "eiaouscrgm", 'l');
    check_guess(&guesser, "lug-orms", "eiaouscrgml", 'w');
}

#[test]
fn trace_sculpturing() {
    let guesser = from_large_file(11);
    check_guess(&guesser, "-----------", "", 'e');
    check_guess(&guesser, "-----------", "e", 'i');
    check_guess(&guesser, "--------i--", "ei", 'n');
    check_guess(&guesser, "--------in-", "ein", 'g');
    check_guess(&guesser, "--------ing", "eing", 'o');
    check_guess(&guesser, "--------ing", "eingo", 'a');
    check_guess(&guesser, "--------ing", "eingoa", 'u');
    check_guess(&guesser, "--u---u-ing", "eingoau", 'c');
    check_guess(&guesser, "-cu---u-ing", "eingoauc", 'l');
    check_guess(&guesser, "-cul--u-ing", "eingoaucl", 'p');
    check_guess(&guesser, "-culp-u-ing", "eingoauclp", 'r');
    check_guess(&guesser, "-culp-uring", "eingoauclpr", 's');
    check_guess(&guesser, "sculp-uring", "eingoauclprs", 't');
}

#[test]
fn trace_cryptographers() {
    let guesser = from_large_file(14);
    check_guess(&guesser, "--------------", "", 'i');
    check_guess(&guesser, "--------------", "i", 'e');
    check_guess(&guesser, "-----------e--", "ie", 's');
    check_guess(&guesser, "-----------e-s", "ies", 'r');
    check_guess(&guesser, "-r-----r---ers", "iesr", 'p');
    check_guess(&guesser, "-r-p---r-p-ers", "iesrp", 'a');
    check_guess(&guesser, "-r-p---rap-ers", "iesrpa", 'c');
    check_guess(&guesser, "cr-p---rap-ers", "iesrpac", 'g');
    check_guess(&guesser, "cr-p--grap-ers", "iesrpacg", 'h');
    check_guess(&guesser, "cr-p--graphers", "iesrpacgh", 'o');
    check_guess(&guesser, "cr-p-ographers", "iesrpacgho", 't');
    check_guess(&guesser, "cr-ptographers", "iesrpacghot", 'y');
}

#[test]
fn trace_monotonousness() {
    let guesser = from_large_file(14);
    check_guess(&guesser, "--------------", "", 'i');
    check_guess(&guesser, "--------------", "i", 'e');
    check_guess(&guesser, "-----------e--", "ie", 's');
    check_guess(&guesser, "---------s-ess", "ies", 'o');
    check_guess(&guesser, "-o-o-o-o-s-ess", "ieso", 'n');
    check_guess(&guesser, "-ono-ono-sness", "ieson", 'm');
    check_guess(&guesser, "mono-ono-sness", "iesonm", 't');
    check_guess(&guesser, "monotono-sness", "iesonmt", 'u');
}

#[test]
fn trace_psychopathology() {
    let guesser = from_large_file(15);
    check_guess(&guesser, "---------------", "", 'i');
    check_guess(&guesser, "---------------", "i", 'e');
    check_guess(&guesser, "---------------", "ie", 'o');
    check_guess(&guesser, "-----o----o-o--", "ieo", 'h');
    check_guess(&guesser, "----ho---ho-o--", "ieoh", 'p');
    check_guess(&guesser, "p---hop--ho-o--", "ieohp", 'y');
    check_guess(&guesser, "p-y-hop--ho-o-y", "ieohpy", 'a');
    check_guess(&guesser, "p-y-hopa-ho-o-y", "ieohpya", 'c');
    check_guess(&guesser, "p-ychopa-ho-o-y", "ieohpyac", 'g');
    check_guess(&guesser, "p-ychopa-ho-ogy", "ieohpyacg", 'l');
    check_guess(&guesser, "p-ychopa-hology", "ieohpyacgl", 's');
    check_guess(&guesser, "psychopa-hology", "ieohpyacgls", 't');
}
