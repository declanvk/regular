use crate::{
    accept::Accept,
    alphabet::{Alphabet, IntoAlphabet},
    error::Error,
    util::VecSet,
};
use core::{fmt, hash::Hash};
use std::collections::HashMap;

mod operations;

/// Backend for the DFA struct.
///
/// Separating the two types allows for an optimized representation of the
/// transition and state storage, which can be the largest part in most DFAs.
///
/// See the DefaultDFAStorage for a sensible default backend.
pub trait DFAStorage<A: Alphabet>: fmt::Debug + Clone {
    /// Type representing a state of the DFA.
    type State: fmt::Debug + Clone + Eq;

    /// Construct a new instance of this storage from the provided alphabet.
    fn from_alphabet(alphabet: A) -> Self;

    /// Return a reference to the alphabet used by this DFA.
    fn alphabet(&self) -> &A;

    /// Return a list of all the valid states of this DFA.
    fn all_states(&self) -> Vec<Self::State>;

    /// Return a list of all transitions of this DFA.
    fn all_transitions(&self) -> Vec<(Self::State, A::Symbol, Self::State)>;

    /// Return `true` if the given state is valid in this DFA.
    fn contains_state(&self, state: &Self::State) -> bool;

    /// Return `Some(end)` if there exists a transition from `current` to `end`
    /// via the given symbol.
    fn transition(&self, current: Self::State, sym: A::Symbol) -> Option<Self::State>;

    /// Return the resulting state from transitioning from the `current` state
    /// given the `sym` symbol.Alphabet
    ///
    /// # Safety
    /// This version of the transition function is exposed so that
    /// implementations can bypass underlying safety checks when specific
    /// variants are upheld.
    /// 1. The `current` state must have been the result of a previous call to
    /// `add_state`.
    /// 2. The `sym` symbol must be a valid symbol contained in  the alphabet
    /// associated with this DFA.
    #[inline]
    unsafe fn transition_unchecked(&self, current: Self::State, sym: A::Symbol) -> Self::State {
        self.transition(current, sym).unwrap()
    }

    /// Return a new unique state.
    fn add_state(&mut self) -> Self::State;

    /// Record the given transition.
    fn add_transition(&mut self, from: Self::State, sym: A::Symbol, to: Self::State);
}

/// A deterministic finite automaton.
#[derive(Debug, Clone)]
pub struct DFA<A: Alphabet, S: DFAStorage<A> = DefaultDFAStorage<A, <A as Alphabet>::Symbol>> {
    accept: VecSet<S::State>,
    dead: Option<S::State>,
    start: S::State,
    storage: S,
}

impl<A, S> DFA<A, S>
where
    S: DFAStorage<A>,
    S::State: Ord,
    A: Alphabet + Clone,
{
    /// The states of DFA that will cause it to accept a string.
    pub fn accept_states(&self) -> &[S::State] {
        self.accept.as_slice()
    }

    /// An optional state that signals early termination of the DFA, used to
    /// represent an error condition.
    pub fn dead_state(&self) -> Option<&S::State> {
        self.dead.as_ref()
    }

    /// The starting state of the DFA.
    pub fn start_state(&self) -> &S::State {
        &self.start
    }

    /// Convert this DFA back into the DFABuilder form.
    pub fn into_builder(self) -> DFABuilder<A, S> {
        DFABuilder {
            storage: self.storage,
            dead: self.dead,
            accept: self.accept,
            start: Some(self.start),
        }
    }
}

impl<A, S> DFA<A, S>
where
    S: DFAStorage<A>,
    S::State: Ord,
    A: Alphabet,
{
    /// Accept or reject a string based on the content of this DFA.
    ///
    /// This will immediately reject any string that contains a symbol that is
    /// not in the given alphabet.
    pub fn accept<I: IntoIterator<Item = A::Symbol>>(&self, string: I) -> bool {
        let mut current = self.start.clone();

        if let Some(dead_state) = self.dead.clone() {
            for sym in string {
                if dead_state == current {
                    break;
                } else {
                    current = match self.storage.transition(current, sym) {
                        Some(next) => next,
                        None => return false,
                    };
                }
            }
        } else {
            for sym in string {
                current = match self.storage.transition(current, sym) {
                    Some(next) => next,
                    None => return false,
                };
            }
        }

        self.accept.contains(&current)
    }

    /// Accept or reject a string based on the content of this DFA, without
    /// performing checks abouts the validity of the string.
    ///
    /// # Safety
    /// Must assert that all symbol values are contained in the alphabet prior
    /// to calling this function.
    pub unsafe fn accept_unchecked<I: IntoIterator<Item = A::Symbol>>(&self, string: I) -> bool {
        let mut current = self.start.clone();

        if let Some(dead_state) = self.dead.clone() {
            for sym in string {
                if dead_state == current {
                    break;
                } else {
                    current = self.storage.transition_unchecked(current, sym);
                }
            }
        } else {
            for sym in string {
                current = self.storage.transition_unchecked(current, sym);
            }
        }

        self.accept.contains(&current)
    }
}

impl<A, S> Accept for &DFA<A, S>
where
    S: DFAStorage<A>,
    S::State: Ord,
    A: Alphabet,
{
    type Symbol = A::Symbol;

    fn accept<I: IntoIterator<Item = Self::Symbol>>(self, string: I) -> bool {
        DFA::accept(self, string)
    }
}

/// Default storage for a DFA.
///
/// Consists of a Hashmap for storing transistions, and the set of states is a
/// linear range.
#[derive(Debug, Clone)]
pub struct DefaultDFAStorage<A: Alphabet, S: Eq + Hash> {
    alphabet: A,
    next_state: usize,
    transition: HashMap<(usize, S), usize>,
}

impl<A: Alphabet> DefaultDFAStorage<A, A::Symbol>
where
    A::Symbol: Eq + Hash,
{
    /// Construct a new default storage with the given alphabet.
    pub fn new(alphabet: A) -> Self {
        DefaultDFAStorage {
            alphabet,
            next_state: 0,
            transition: HashMap::new(),
        }
    }
}

impl<A> DFAStorage<A> for DefaultDFAStorage<A, A::Symbol>
where
    A: Alphabet + fmt::Debug + Clone,
    A::Symbol: fmt::Debug + Clone + Eq + Hash,
{
    type State = usize;

    fn from_alphabet(alphabet: A) -> Self {
        Self::new(alphabet)
    }

    fn all_states(&self) -> Vec<Self::State> {
        (0..self.next_state).collect()
    }

    fn all_transitions(&self) -> Vec<(Self::State, A::Symbol, Self::State)> {
        self.transition
            .iter()
            .map(|((from, sym), to)| (*from, sym.clone(), *to))
            .collect()
    }

    fn transition(&self, current: Self::State, sym: A::Symbol) -> Option<Self::State> {
        self.transition.get(&(current, sym)).cloned()
    }

    fn add_state(&mut self) -> Self::State {
        let new_state = self.next_state;
        self.next_state += 1;

        new_state
    }

    fn add_transition(&mut self, from: Self::State, sym: A::Symbol, to: Self::State) {
        self.transition.insert((from, sym), to);
    }

    fn contains_state(&self, state: &Self::State) -> bool {
        *state < self.next_state
    }

    fn alphabet(&self) -> &A {
        &self.alphabet
    }
}

/// Builder for a DFA.
#[derive(Debug, Clone)]
pub struct DFABuilder<A: Alphabet, S: DFAStorage<A> = DefaultDFAStorage<A, <A as Alphabet>::Symbol>>
{
    accept: VecSet<S::State>,
    dead: Option<S::State>,
    start: Option<S::State>,
    storage: S,
}

impl<A> DFABuilder<A>
where
    A: Alphabet + fmt::Debug + Clone,
    A::Symbol: Eq + Hash + fmt::Debug + Clone,
{
    /// Create a new DFABuilder with the given alphabet.
    pub fn new<I: IntoAlphabet<IntoAlpha = A, Symbol = A::Symbol>>(alphabet: I) -> Self {
        DFABuilder {
            accept: VecSet::new(),
            dead: None,
            start: None,
            storage: DefaultDFAStorage::new(alphabet.into_alphabet()),
        }
    }
}

impl<A, S> DFABuilder<A, S>
where
    S: DFAStorage<A>,
    S::State: Ord,
    A: Alphabet,
{
    /// Create a new DFABuilder with a customer storage backend.
    pub fn new_with_storage(storage: S) -> Self {
        DFABuilder {
            storage,
            accept: VecSet::new(),
            dead: None,
            start: None,
        }
    }

    /// Record and return a new state.
    pub fn new_state(&mut self) -> S::State {
        self.storage.add_state()
    }

    /// Record and validate a new transition.
    ///
    /// # Error
    ///
    /// This function will error if the symbol was not a member of the stated
    /// alphabet. This function will error if either the `from` or `to` state is
    /// not a valid state (valid states are only returned from calls of the
    /// `add_state` function).
    pub fn transition(
        &mut self,
        from: S::State,
        sym: A::Symbol,
        to: S::State,
    ) -> Result<(), Error> {
        if !self.storage.contains_state(&from) || !self.storage.contains_state(&to) {
            Err(Error::InvalidState)
        } else if !self.storage.alphabet().contains(&sym) {
            Err(Error::SymbolNotInAlphabet)
        } else {
            self.storage.add_transition(from, sym, to);

            Ok(())
        }
    }

    /// Record and validate multiple transitions.
    ///
    /// # Error
    ///
    /// See the Error documentation of `transition` for ways that this function
    /// can fail.
    pub fn transitions(
        &mut self,
        transitions: impl IntoIterator<Item = (S::State, A::Symbol, S::State)>,
    ) -> Result<(), Error> {
        for (from, sym, to) in transitions {
            self.transition(from, sym, to)?;
        }

        Ok(())
    }

    /// Add to the set of accept states.
    pub fn accept_states(
        &mut self,
        accept_states: impl IntoIterator<Item = S::State>,
    ) -> &mut Self {
        self.accept.extend(accept_states);

        self
    }

    /// Set the starting state.
    pub fn start_state(&mut self, start: S::State) -> &mut Self {
        self.start = Some(start);

        self
    }

    /// Set the dead/error state.
    pub fn dead_state(&mut self, dead: Option<S::State>) -> &mut Self {
        self.dead = dead;

        self
    }

    /// Build the
    pub fn build(self) -> Result<DFA<A, S>, Error> {
        let DFABuilder {
            start,
            storage,
            dead,
            accept,
        } = self;
        let start = start.ok_or(Error::MissingStartState)?;

        if dead
            .as_ref()
            .map_or(false, |dead| !storage.contains_state(dead))
        {
            return Err(Error::InvalidState);
        }

        for state in &accept {
            if !storage.contains_state(state) {
                return Err(Error::InvalidState);
            }
        }

        Ok(DFA {
            start,
            dead,
            accept,
            storage,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{accept::IterExt, util::Range};

    // Accept: a*b*c*
    // Transitions: (from_state) -<symbol>- (to_state)
    // 0 -<a>- 0
    // 0 -<b>- 1
    // 0 -<c>- 2
    // 1 -<a>- dead
    // 1 -<b>- 1
    // 1 -<c>- 2
    // 2 -<a>- dead
    // 2 -<b>- dead
    // 2 -<c>- 2
    // 0 is start, {0, 1, 2} are accept
    fn simple_dfa() -> DFA<Range<char>, DefaultDFAStorage<Range<char>, char>> {
        let mut builder = DFABuilder::new('a'..='c');
        let s0 = builder.new_state();
        let s1 = builder.new_state();
        let s2 = builder.new_state();
        let sdead = builder.new_state();

        builder
            .transitions(
                [
                    (s0, 'a', s0),
                    (s0, 'b', s1),
                    (s0, 'c', s2),
                    (s1, 'a', sdead),
                    (s1, 'b', s1),
                    (s1, 'c', s2),
                    (s2, 'a', sdead),
                    (s2, 'b', sdead),
                    (s2, 'c', s2),
                ]
                .iter()
                .copied(),
            )
            .unwrap();

        builder
            .start_state(s0)
            .dead_state(Some(sdead))
            .accept_states([s0, s1, s2].iter().copied());
        builder.build().expect("DFA construction failed!")
    }

    #[test]
    fn accept_by_simple_dfa() {
        let dfa = simple_dfa();

        assert!(dfa.accept("aaaabbbbcccc".chars()));
        assert!(dfa.accept("abc".chars()));
        assert!(dfa.accept("bbcc".chars()));
        assert!(dfa.accept("cc".chars()));
        assert!(dfa.accept("aacc".chars()));
        assert!(dfa.accept("".chars()));
        assert!(dfa.accept("aabb".chars()));
        assert!(dfa.accept("bb".chars()));
        assert!(dfa.accept("aa".chars()));
        assert!("aaaabbbbcccc".chars().is_accepted(&dfa));

        assert!(!dfa.accept("cbbbbcccc".chars()));
        assert!(!dfa.accept("z".chars()));
        assert!(!dfa.accept("ccbbaa".chars()));
        assert!(!dfa.accept("abbaa".chars()));
        assert!(!dfa.accept("abcbaa".chars()));
    }

    #[test]
    fn accept_by_complement_simple_dfa() {
        let dfa = simple_dfa().complement();

        assert!(!dfa.accept("aaaabbbbcccc".chars()));
        assert!(!dfa.accept("abc".chars()));
        assert!(!dfa.accept("bbcc".chars()));
        assert!(!dfa.accept("cc".chars()));
        assert!(!dfa.accept("aacc".chars()));
        assert!(!dfa.accept("".chars()));
        assert!(!dfa.accept("aabb".chars()));
        assert!(!dfa.accept("bb".chars()));
        assert!(!dfa.accept("aa".chars()));
        assert!(!"aaaabbbbcccc".chars().is_accepted(&dfa));

        // Still don't accept string with non-alphabet symbols
        assert!(!dfa.accept("z".chars()));

        assert!(dfa.accept("cbbbbcccc".chars()));
        assert!(dfa.accept("ccbbaa".chars()));
        assert!(dfa.accept("abbaa".chars()));
        assert!(dfa.accept("abcbaa".chars()));
    }
}
