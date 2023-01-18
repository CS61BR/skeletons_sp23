use crate::choosers::Chooser;

mod evil_chooser;
mod letter_freq_guesser;
mod paga_lfg;
mod pattern_aware_lfg;
mod random_chooser;

fn test_trace<C: Chooser>(mut chooser: C, trace: Vec<(char, usize, &str)>) {
    for (input, output, pat) in &trace {
        assert_eq!(chooser.make_guess(*input), *output);
        assert_eq!(chooser.get_pattern(), *pat);
    }
}
