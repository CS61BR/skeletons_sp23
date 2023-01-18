use crate::guessers::{letter_freq_guesser::LetterFreqGuesser, Guesser};

fn check_guess_order(words: Vec<String>, correct_order: &str) {
    let pattern = "-".repeat(words[0].len());
    let guesser = LetterFreqGuesser::new(words);
    let mut guesses = Vec::new();
    for a in correct_order.chars() {
        assert_eq!(guesser.guess(&pattern, &guesses), a);
        guesses.push(a);
    }
}

#[test]
fn foods() {
    check_guess_order(vec!["cheese".to_owned()], "echs");
    check_guess_order(vec!["blueberry".to_owned()], "berluy");
    check_guess_order(vec!["potato".to_owned()], "otap");
}

#[test]
fn long_word() {
    // This test uses one very very long word
    let correct_order = "cgoebtahrxqyklpusvjdwinmfz";
    let words = vec!["csvxyddebsohjhtghhgrbyxvnxohsubyhrcqoyqlcoux\
    gletgtgtcygkqjgewtlefcwpcvavaqbatyjoeooyqhecwcaxxsdycugoubrhkhvbhexuboxbcxt\
    ahdqpbleqtakdpoopicllttkioggatrkonsgexroiabeecwggfcbvxermbcjeyuhlrgqeyxaetr\
    uvckxoapkkcghlbjebyicqabyjjtoxtrbuhhplpacanuwogrknaqzlosraxphcpxmqgkdageber\
    gldbqopehrqyyhkrivabkgvycocbqeeltktuaeajqgttxsbqhgrtcageorsccgsorwgmpclkpcs\
    rouottb"
        .to_owned()];
    check_guess_order(words, correct_order);
}

#[test]
fn more_foods() {
    let correct_order = "ebortachlpsuy";
    let words = vec![
        "cheese".to_owned(),
        "blueberry".to_owned(),
        "potato".to_owned(),
    ];
    check_guess_order(words, correct_order)
}

#[test]
fn random_words() {
    let correct_order = "mvyfbocwgiasklptjnxhqrdeu";
    let words = vec![
        "vwafymnidtapkfcqowrglybusuvjfgmchmfwnmkblbijpwnpta".to_owned(),
        "ofobbbbsmitmeyfgqsodioogyyocqxjqkfjpchlvfkavcvjufi".to_owned(),
        "vwpmenvdcsmmagrxiknxjbywoqgaiopxtrsytnpclblcsvmywm".to_owned(),
        "kqlgelrhrhfkaswwhiyfvxseagebmgvmtgvsqcinyosjlatpwm".to_owned(),
        "xtldayhcwmvwvvfbhbfcbvmvyvrdxtimgxiokyyojonpymfkcv".to_owned(),
    ];
    check_guess_order(words, correct_order);
}
