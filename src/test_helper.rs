use crate::{
    alphabet::Boolean,
    dfa::{DFABuilder, DFA},
};
use core::iter::once;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn convert_string<T: Clone>(raw: String, convert: HashMap<char, T>) -> Vec<T> {
    raw.chars()
        .filter_map(|c| convert.get(&c))
        .cloned()
        .collect()
}

pub fn binary_converter() -> HashMap<char, bool> {
    [('0', false), ('1', true)].iter().cloned().collect()
}

pub static CONTAINS_TWO_FALSE_DFA: Lazy<DFA<Boolean>> = Lazy::new(|| {
    let mut builder = DFABuilder::new(Boolean);

    let q0 = builder.new_state();
    let q1 = builder.new_state();
    let q2 = builder.new_state();

    builder
        .transitions(
            [
                (q0, false, q1),
                (q0, true, q0),
                (q1, false, q2),
                (q1, true, q0),
                (q2, false, q2),
                (q2, true, q2),
            ]
            .iter()
            .cloned(),
        )
        .unwrap();

    builder
        .start_state(q0)
        .accept_states(once(q2))
        .dead_state(Some(q2));

    builder.build().unwrap()
});

pub static CONTAINS_EVEN_TRUES_DFA: Lazy<DFA<Boolean>> = Lazy::new(|| {
    let mut builder = DFABuilder::new(Boolean);

    let q0 = builder.new_state();
    let q1 = builder.new_state();

    builder
        .transitions(
            [
                (q0, false, q0),
                (q0, true, q1),
                (q1, false, q1),
                (q1, true, q0),
            ]
            .iter()
            .cloned(),
        )
        .unwrap();

    builder.start_state(q0).accept_states(once(q0));

    builder.build().unwrap()
});

pub static CONTAINS_EVEN_TRUES_OR_TWO_FALSE_DFA: Lazy<DFA<Boolean>> = Lazy::new(|| {
    CONTAINS_EVEN_TRUES_DFA
        .union(&CONTAINS_TWO_FALSE_DFA)
        .unwrap()
});
