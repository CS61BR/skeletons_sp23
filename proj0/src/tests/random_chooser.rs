use std::collections::HashMap;

use crate::{
    choosers::{random_chooser::RandomChooser, Chooser},
    random::Random,
};

use super::test_trace;

fn choose_word(s: &str) -> RandomChooser {
    RandomChooser::new(vec![s.to_owned()], Random::new(""))
}

/// Since RandomChooser should choose words uniformly, check that with 7 possible words,
/// each one gets chosen at least 100 out of 1000 times. On average, each word
/// should end up with around 143, so 100 is very conservative
#[test]
fn uses_random() {
    let possible_words = Vec::from(["a", "b", "c", "d", "e", "f", "g"].map(|s| s.to_owned()));

    let mut counts = HashMap::new();
    for w in &possible_words {
        counts.insert(w.to_owned(), 0);
    }
    for x in 0..1000 {
        let c = RandomChooser::new(possible_words.clone(), Random::new(&format!("{x}")));
        *counts.get_mut(c.get_word()).unwrap() += 1;
    }
    for w in &possible_words {
        assert!(*counts.get(w).unwrap() > 100);
    }

    print!("{:?}", counts);
}

#[test]
fn trace_cheese() {
    test_trace(
        choose_word("cheese"),
        vec![
            ('q', 0, "------"),
            ('a', 0, "------"),
            ('v', 0, "------"),
            ('k', 0, "------"),
            ('l', 0, "------"),
            ('e', 3, "--ee-e"),
            ('m', 0, "--ee-e"),
            ('i', 0, "--ee-e"),
            ('b', 0, "--ee-e"),
            ('g', 0, "--ee-e"),
            ('x', 0, "--ee-e"),
            ('o', 0, "--ee-e"),
            ('z', 0, "--ee-e"),
            ('j', 0, "--ee-e"),
            ('f', 0, "--ee-e"),
            ('t', 0, "--ee-e"),
            ('y', 0, "--ee-e"),
            ('s', 1, "--eese"),
            ('d', 0, "--eese"),
            ('w', 0, "--eese"),
            ('c', 1, "c-eese"),
            ('u', 0, "c-eese"),
            ('n', 0, "c-eese"),
            ('r', 0, "c-eese"),
            ('h', 1, "cheese"),
        ],
    );
}

#[test]
fn trace_potato() {
    test_trace(
        choose_word("potato"),
        vec![
            ('p', 1, "p-----"),
            ('h', 0, "p-----"),
            ('n', 0, "p-----"),
            ('f', 0, "p-----"),
            ('c', 0, "p-----"),
            ('t', 2, "p-t-t-"),
            ('j', 0, "p-t-t-"),
            ('v', 0, "p-t-t-"),
            ('r', 0, "p-t-t-"),
            ('g', 0, "p-t-t-"),
            ('l', 0, "p-t-t-"),
            ('x', 0, "p-t-t-"),
            ('b', 0, "p-t-t-"),
            ('d', 0, "p-t-t-"),
            ('e', 0, "p-t-t-"),
            ('q', 0, "p-t-t-"),
            ('y', 0, "p-t-t-"),
            ('w', 0, "p-t-t-"),
            ('i', 0, "p-t-t-"),
            ('a', 1, "p-tat-"),
            ('k', 0, "p-tat-"),
            ('u', 0, "p-tat-"),
            ('z', 0, "p-tat-"),
            ('s', 0, "p-tat-"),
            ('o', 2, "potato"),
        ],
    );
}

#[test]
fn trace_hippopotomonstrosesquippedaliophobia() {
    test_trace(
        choose_word("hippopotomonstrosesquippedaliophobia"),
        vec![
            ('w', 0, "------------------------------------"),
            ('v', 0, "------------------------------------"),
            ('z', 0, "------------------------------------"),
            ('x', 0, "------------------------------------"),
            ('a', 2, "--------------------------a--------a"),
            ('g', 0, "--------------------------a--------a"),
            ('r', 1, "--------------r-----------a--------a"),
            ('d', 1, "--------------r----------da--------a"),
            ('f', 0, "--------------r----------da--------a"),
            ('m', 1, "---------m----r----------da--------a"),
            ('n', 1, "---------m-n--r----------da--------a"),
            ('o', 7, "----o-o-omon--ro---------da--o--o--a"),
            ('q', 1, "----o-o-omon--ro---q-----da--o--o--a"),
            ('c', 0, "----o-o-omon--ro---q-----da--o--o--a"),
            ('j', 0, "----o-o-omon--ro---q-----da--o--o--a"),
            ('e', 2, "----o-o-omon--ro-e-q----eda--o--o--a"),
            ('i', 4, "-i--o-o-omon--ro-e-q-i--eda-io--o-ia"),
            ('h', 2, "hi--o-o-omon--ro-e-q-i--eda-io-ho-ia"),
            ('k', 0, "hi--o-o-omon--ro-e-q-i--eda-io-ho-ia"),
            ('s', 3, "hi--o-o-omons-rosesq-i--eda-io-ho-ia"),
            ('p', 6, "hippopo-omons-rosesq-ippeda-iopho-ia"),
            ('y', 0, "hippopo-omons-rosesq-ippeda-iopho-ia"),
            ('t', 2, "hippopotomonstrosesq-ippeda-iopho-ia"),
            ('b', 1, "hippopotomonstrosesq-ippeda-iophobia"),
            ('u', 1, "hippopotomonstrosesquippeda-iophobia"),
            ('l', 1, "hippopotomonstrosesquippedaliophobia"),
        ],
    );
}

#[test]
fn trace_incomprehensibility() {
    test_trace(
        choose_word("incomprehensibility"),
        vec![
            ('u', 0, "-------------------"),
            ('d', 0, "-------------------"),
            ('z', 0, "-------------------"),
            ('r', 1, "------r------------"),
            ('p', 1, "-----pr------------"),
            ('y', 1, "-----pr-----------y"),
            ('c', 1, "--c--pr-----------y"),
            ('k', 0, "--c--pr-----------y"),
            ('b', 1, "--c--pr------b----y"),
            ('w', 0, "--c--pr------b----y"),
            ('m', 1, "--c-mpr------b----y"),
            ('n', 2, "-nc-mpr---n--b----y"),
            ('o', 1, "-ncompr---n--b----y"),
            ('s', 1, "-ncompr---ns-b----y"),
            ('l', 1, "-ncompr---ns-b-l--y"),
            ('a', 0, "-ncompr---ns-b-l--y"),
            ('q', 0, "-ncompr---ns-b-l--y"),
            ('t', 1, "-ncompr---ns-b-l-ty"),
            ('i', 4, "incompr---nsibility"),
            ('v', 0, "incompr---nsibility"),
            ('h', 1, "incompr-h-nsibility"),
            ('e', 2, "incomprehensibility"),
        ],
    );
}

#[test]
fn trace_a() {
    test_trace(choose_word("a"), vec![('a', 1, "a")]);
}

#[test]
fn trace_i() {
    test_trace(
        choose_word("i"),
        vec![
            ('a', 0, "-"),
            ('o', 0, "-"),
            ('z', 0, "-"),
            ('b', 0, "-"),
            ('t', 0, "-"),
            ('y', 0, "-"),
            ('m', 0, "-"),
            ('g', 0, "-"),
            ('s', 0, "-"),
            ('u', 0, "-"),
            ('c', 0, "-"),
            ('h', 0, "-"),
            ('j', 0, "-"),
            ('i', 1, "i"),
        ],
    );
}

#[test]
fn trace_beans() {
    test_trace(
        choose_word("beans"),
        vec![
            ('l', 0, "-----"),
            ('h', 0, "-----"),
            ('s', 1, "----s"),
            ('u', 0, "----s"),
            ('v', 0, "----s"),
            ('m', 0, "----s"),
            ('a', 1, "--a-s"),
            ('r', 0, "--a-s"),
            ('f', 0, "--a-s"),
            ('g', 0, "--a-s"),
            ('t', 0, "--a-s"),
            ('d', 0, "--a-s"),
            ('q', 0, "--a-s"),
            ('o', 0, "--a-s"),
            ('c', 0, "--a-s"),
            ('x', 0, "--a-s"),
            ('z', 0, "--a-s"),
            ('j', 0, "--a-s"),
            ('w', 0, "--a-s"),
            ('k', 0, "--a-s"),
            ('b', 1, "b-a-s"),
            ('y', 0, "b-a-s"),
            ('p', 0, "b-a-s"),
            ('i', 0, "b-a-s"),
            ('n', 1, "b-ans"),
            ('e', 1, "beans"),
        ],
    );
}

#[test]
fn trace_pneumonoultramicroscopicsilicovolcanoconiosis() {
    test_trace(
        choose_word("pneumonoultramicroscopicsilicovolcanoconiosis"),
        vec![
            ('g', 0, "---------------------------------------------"),
            ('f', 0, "---------------------------------------------"),
            ('o', 9, "-----o-o---------o--o--------o-o----o-o--o---"),
            ('t', 1, "-----o-o--t------o--o--------o-o----o-o--o---"),
            ('i', 6, "-----o-o--t---i--o--o-i--i-i-o-o----o-o-io-i-"),
            ('n', 4, "-n---ono--t---i--o--o-i--i-i-o-o---no-onio-i-"),
            ('c', 6, "-n---ono--t---ic-o-co-ic-i-ico-o-c-noconio-i-"),
            ('v', 1, "-n---ono--t---ic-o-co-ic-i-icovo-c-noconio-i-"),
            ('d', 0, "-n---ono--t---ic-o-co-ic-i-icovo-c-noconio-i-"),
            ('j', 0, "-n---ono--t---ic-o-co-ic-i-icovo-c-noconio-i-"),
            ('e', 1, "-ne--ono--t---ic-o-co-ic-i-icovo-c-noconio-i-"),
            ('u', 2, "-neu-onou-t---ic-o-co-ic-i-icovo-c-noconio-i-"),
            ('m', 2, "-neumonou-t--mic-o-co-ic-i-icovo-c-noconio-i-"),
            ('l', 3, "-neumonoult--mic-o-co-ic-ilicovolc-noconio-i-"),
            ('z', 0, "-neumonoult--mic-o-co-ic-ilicovolc-noconio-i-"),
            ('w', 0, "-neumonoult--mic-o-co-ic-ilicovolc-noconio-i-"),
            ('a', 2, "-neumonoult-amic-o-co-ic-ilicovolcanoconio-i-"),
            ('b', 0, "-neumonoult-amic-o-co-ic-ilicovolcanoconio-i-"),
            ('h', 0, "-neumonoult-amic-o-co-ic-ilicovolcanoconio-i-"),
            ('x', 0, "-neumonoult-amic-o-co-ic-ilicovolcanoconio-i-"),
            ('s', 4, "-neumonoult-amic-osco-icsilicovolcanoconiosis"),
            ('k', 0, "-neumonoult-amic-osco-icsilicovolcanoconiosis"),
            ('q', 0, "-neumonoult-amic-osco-icsilicovolcanoconiosis"),
            ('r', 2, "-neumonoultramicrosco-icsilicovolcanoconiosis"),
            ('p', 2, "pneumonoultramicroscopicsilicovolcanoconiosis"),
        ],
    );
}

#[test]
fn trace_pointlessness() {
    test_trace(
        choose_word("pointlessness"),
        vec![
            ('o', 1, "-o-----------"),
            ('z', 0, "-o-----------"),
            ('w', 0, "-o-----------"),
            ('p', 1, "po-----------"),
            ('t', 1, "po--t--------"),
            ('y', 0, "po--t--------"),
            ('d', 0, "po--t--------"),
            ('i', 1, "poi-t--------"),
            ('q', 0, "poi-t--------"),
            ('g', 0, "poi-t--------"),
            ('v', 0, "poi-t--------"),
            ('n', 2, "point----n---"),
            ('u', 0, "point----n---"),
            ('f', 0, "point----n---"),
            ('j', 0, "point----n---"),
            ('l', 1, "pointl---n---"),
            ('s', 4, "pointl-ssn-ss"),
            ('a', 0, "pointl-ssn-ss"),
            ('m', 0, "pointl-ssn-ss"),
            ('c', 0, "pointl-ssn-ss"),
            ('r', 0, "pointl-ssn-ss"),
            ('e', 2, "pointlessness"),
        ],
    );
}

#[test]
fn trace_celticstudies() {
    test_trace(
        choose_word("celticstudies"),
        vec![
            ('r', 0, "-------------"),
            ('k', 0, "-------------"),
            ('j', 0, "-------------"),
            ('w', 0, "-------------"),
            ('h', 0, "-------------"),
            ('o', 0, "-------------"),
            ('i', 2, "----i-----i--"),
            ('d', 1, "----i----di--"),
            ('x', 0, "----i----di--"),
            ('f', 0, "----i----di--"),
            ('z', 0, "----i----di--"),
            ('m', 0, "----i----di--"),
            ('s', 2, "----i-s--di-s"),
            ('g', 0, "----i-s--di-s"),
            ('c', 2, "c---ics--di-s"),
            ('t', 2, "c--ticst-di-s"),
            ('b', 0, "c--ticst-di-s"),
            ('e', 2, "ce-ticst-dies"),
            ('l', 1, "celticst-dies"),
            ('u', 1, "celticstudies"),
        ],
    );
}

#[test]
fn trace_eecs() {
    test_trace(
        choose_word("eecs"),
        vec![
            ('e', 2, "ee--"),
            ('z', 0, "ee--"),
            ('y', 0, "ee--"),
            ('u', 0, "ee--"),
            ('q', 0, "ee--"),
            ('c', 1, "eec-"),
            ('n', 0, "eec-"),
            ('v', 0, "eec-"),
            ('s', 1, "eecs"),
        ],
    );
}

#[test]
fn trace_poetry() {
    test_trace(
        choose_word("poetry"),
        vec![
            ('i', 0, "------"),
            ('x', 0, "------"),
            ('n', 0, "------"),
            ('q', 0, "------"),
            ('j', 0, "------"),
            ('m', 0, "------"),
            ('h', 0, "------"),
            ('v', 0, "------"),
            ('f', 0, "------"),
            ('t', 1, "---t--"),
            ('g', 0, "---t--"),
            ('u', 0, "---t--"),
            ('d', 0, "---t--"),
            ('c', 0, "---t--"),
            ('o', 1, "-o-t--"),
            ('p', 1, "po-t--"),
            ('b', 0, "po-t--"),
            ('s', 0, "po-t--"),
            ('r', 1, "po-tr-"),
            ('z', 0, "po-tr-"),
            ('a', 0, "po-tr-"),
            ('e', 1, "poetr-"),
            ('l', 0, "poetr-"),
            ('k', 0, "poetr-"),
            ('w', 0, "poetr-"),
            ('y', 1, "poetry"),
        ],
    );
}

#[test]
fn trace_fractionalization() {
    test_trace(
        choose_word("fractionalization"),
        vec![
            ('v', 0, "-----------------"),
            ('e', 0, "-----------------"),
            ('l', 1, "---------l-------"),
            ('t', 2, "----t----l---t---"),
            ('a', 3, "--a-t---al--at---"),
            ('p', 0, "--a-t---al--at---"),
            ('n', 2, "--a-t--nal--at--n"),
            ('o', 2, "--a-t-onal--at-on"),
            ('w', 0, "--a-t-onal--at-on"),
            ('m', 0, "--a-t-onal--at-on"),
            ('y', 0, "--a-t-onal--at-on"),
            ('c', 1, "--act-onal--at-on"),
            ('q', 0, "--act-onal--at-on"),
            ('z', 1, "--act-onal-zat-on"),
            ('s', 0, "--act-onal-zat-on"),
            ('x', 0, "--act-onal-zat-on"),
            ('j', 0, "--act-onal-zat-on"),
            ('g', 0, "--act-onal-zat-on"),
            ('k', 0, "--act-onal-zat-on"),
            ('i', 3, "--actionalization"),
            ('h', 0, "--actionalization"),
            ('f', 1, "f-actionalization"),
            ('u', 0, "f-actionalization"),
            ('r', 1, "fractionalization"),
        ],
    );
}

#[test]
fn trace_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa() {
    test_trace(
        choose_word("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        vec![
            ('t', 0, "------------------------------------------"),
            ('y', 0, "------------------------------------------"),
            ('a', 42, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        ],
    );
}

#[test]
fn trace_blueberry() {
    test_trace(
        choose_word("blueberry"),
        vec![
            ('z', 0, "---------"),
            ('c', 0, "---------"),
            ('t', 0, "---------"),
            ('o', 0, "---------"),
            ('r', 2, "------rr-"),
            ('f', 0, "------rr-"),
            ('m', 0, "------rr-"),
            ('p', 0, "------rr-"),
            ('s', 0, "------rr-"),
            ('a', 0, "------rr-"),
            ('w', 0, "------rr-"),
            ('i', 0, "------rr-"),
            ('e', 2, "---e-err-"),
            ('h', 0, "---e-err-"),
            ('x', 0, "---e-err-"),
            ('k', 0, "---e-err-"),
            ('l', 1, "-l-e-err-"),
            ('b', 2, "bl-eberr-"),
            ('y', 1, "bl-eberry"),
            ('u', 1, "blueberry"),
        ],
    );
}
