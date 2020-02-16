#![deny(missing_docs, clippy::missing_safety_doc)]

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

pub use dfa::{DFABuilder, DFAStorage, DefaultDFAStorage, DFA};
pub use error::Error;

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
