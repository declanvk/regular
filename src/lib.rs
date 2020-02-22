#![deny(missing_docs, clippy::missing_safety_doc)]

//! Tools for manipulating regular languages.
//!
//! # Papers of interest
//!
//! EFFICIENT MINIMIZATION OF DFAS WITH PARTIAL TRANSITION FUNCTIONS
//!   - Antti Valmari & Petri Lehtinen
//!   - https://arxiv.org/pdf/0802.2826.pdf
//!   - this one is interesting because the presence of a dead state means that
//!     the transition function is essentially partial, by the definition used
//!     in the paper
//!
//! Fast brief practical DFA minimization
//!   - Antti Valmari
//!   - https://sci-hub.se/10.1016/j.ipl.2011.12.004
//!
//! The Practical Performance of Automata Minimization Algorithms
//!   - Erin van der Veen
//!   - https://www.cs.ru.nl/bachelors-theses/2017/Erin_van_der_Veen___4431200___The_Practical_Performance_of_Automata_Minimization_Algorithms.pdf
//!
//! Treatment of Epsilon Moves in Subset Construction
//!   - Gertjan van Noord
//!   - https://sci-hub.se/10.1162/089120100561638
//!
//! Generic epsilon-removal
//!   - Vivien Delmon
//!   - https://www.lrde.epita.fr/dload/20070523-Seminar/delmon-eps-removal-vcsn-report.pdf
//!
//! Effcient Approaches to Subset Construction
//!   - Ted Leslie
//!   - http://citeseerx.ist.psu.edu/viewdoc/download;jsessionid=318E0FC96DD030FC1E7A1AE52148DD83?doi=10.1.1.8.7435&rep=rep1&type=pdf
//!   - This paper is not very clear, but if you can decipher it then it holds
//!     an efficient implementation and description of data structures for
//!     subset construction (determinization)

/// Generalization of the accept/non-accept of regular expressions, DFAs, and
/// NFAs. Connected to the larger concept of recognizing that some string
/// belongs to a language.
pub mod accept;
/// Traits and implementations of generic sets of symbol, called alphabets in
/// the context of languages and strings.
pub mod alphabet;
/// Implementation of discrete finite automaton.
pub(crate) mod dfa;
pub(crate) mod error;
pub(crate) mod util;

#[cfg(test)]
pub(crate) mod test_helper;

pub use dfa::{DFABuilder, DFAStorage, DefaultDFAStorage, DFA};
pub use error::Error;
pub use util::Range;

/// Common items to import.
///
/// # Example
/// ```
/// use regular::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        accept::{Accept, IterExt},
        dfa::{DFABuilder, DFAStorage, DefaultDFAStorage, DFA},
        error::Error,
    };
}
