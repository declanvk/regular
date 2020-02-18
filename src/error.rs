use core::fmt;

/// Errors from `regular` operations.
#[derive(Debug, Clone)]
pub enum Error {
    /// Start state was not specified.
    MissingStartState,
    /// State specified was not valid for this automaton.
    InvalidState,
    /// Symbol not found in alphabet.
    SymbolNotInAlphabet,
    /// State not found.
    StateNotFound,
    /// Attempted to perform operation with two different alphabets.
    OperationWithNonEqualAlphabets,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingStartState => write!(f, "Start state was not specified."),
            Error::InvalidState => write!(f, "State specified was not valid for this automaton."),
            Error::SymbolNotInAlphabet => write!(f, "Symbol not found in alphabet."),
            Error::OperationWithNonEqualAlphabets => write!(
                f,
                "Attempted to perform operation with two different alphabets."
            ),
            Error::StateNotFound => write!(f, "State not found."),
        }
    }
}
