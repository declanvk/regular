use crate::{
    alphabet::Alphabet,
    dfa::{DFAStorage, DFA},
    error::Error,
    util::{CartesianProductIter, VecSet},
};
use core::hash::Hash;
use std::collections::HashMap;

// Create a new DFA that is the cross product construction of the two given
// DFAs, without initializing the accept states of the DFA.
fn cross_product_construction_without_accept_states<SL, SR, SN, A>(
    left: &DFA<A, SL>,
    right: &DFA<A, SR>,
) -> Result<
    (
        SN::State,
        Option<SN::State>,
        SN,
        HashMap<(SL::State, SR::State), SN::State>,
    ),
    Error,
>
where
    SL: DFAStorage<A>,
    SL::State: Ord + Hash,

    SR: DFAStorage<A>,
    SR::State: Ord + Hash,

    SN: DFAStorage<A>,
    SN::State: Ord,

    A: Alphabet + PartialEq + Clone,
    A::Symbol: Clone,
{
    if left.storage.alphabet() != right.storage.alphabet() {
        return Err(Error::OperationWithNonEqualAlphabets);
    }

    let alphabet = left.storage.alphabet().clone();

    let mut new_storage = SN::from_alphabet(alphabet);
    let mut state_mapping: HashMap<(SL::State, SR::State), SN::State> = HashMap::new();

    for left_state in left.storage.all_states() {
        for right_state in right.storage.all_states() {
            let new_state = new_storage.add_state();

            state_mapping.insert((left_state.clone(), right_state), new_state);
        }
    }

    let new_start: SN::State = state_mapping
        .get(&(left.start.clone(), right.start.clone()))
        .ok_or(Error::StateNotFound)?
        .clone();
    let new_dead: Option<SN::State> = left
        .dead
        .as_ref()
        .cloned()
        .and_then(|left_dead| {
            right
                .dead
                .as_ref()
                .cloned()
                .map(|right_dead| (left_dead, right_dead))
        })
        .and_then(|dead_pair| state_mapping.get(&dead_pair).cloned());

    for ((self_state, other_state), new_state) in &state_mapping {
        for sym in new_storage.alphabet().values() {
            // This is safe bc self_state and sym both originate indirectly from
            // left.storage
            let self_next = unsafe {
                left.storage
                    .transition_unchecked(self_state.clone(), sym.clone())
            };

            // This is safe bc other_state originates indirectly from other.storage. `sym`
            // is safe to use here bc we asserted that `self`'s alphabet and `other`'s
            // alphabet were the same at the top of this function.
            let other_next = unsafe {
                right
                    .storage
                    .transition_unchecked(other_state.clone(), sym.clone())
            };

            let new_next = state_mapping
                .get(&(self_next, other_next))
                .ok_or(Error::StateNotFound)?;

            new_storage.add_transition(new_state.clone(), sym, new_next.clone());
        }
    }

    Ok((new_start, new_dead, new_storage, state_mapping))
}

impl<A, S> DFA<A, S>
where
    S: DFAStorage<A>,
    S::State: Ord,
    A: Alphabet + Clone,
{
    /// Construct a new DFA that accepts the regular language that is the
    /// intersection of the regular languages represented by this DFA and
    /// another DFA.
    pub fn intersection<S2, S3>(&self, other: &DFA<A, S2>) -> Result<DFA<A, S3>, Error>
    where
        S::State: Hash,
        S2: DFAStorage<A>,
        S2::State: Ord + Hash,
        S3: DFAStorage<A>,
        S3::State: Ord,
        A: PartialEq + Clone,
        A::Symbol: Clone,
    {
        let (new_start, new_dead, new_storage, state_mapping) =
            cross_product_construction_without_accept_states::<S, S2, S3, A>(self, other)?;

        // The intersection accept states is the cartesian product of the previous
        // accept sets
        let mut new_accept: VecSet<S3::State> = VecSet::new();
        for self_accept_state in self.accept.iter().cloned() {
            for other_accept_state in other.accept.iter().cloned() {
                new_accept.insert(
                    state_mapping
                        .get(&(self_accept_state.clone(), other_accept_state))
                        .ok_or(Error::StateNotFound)?
                        .clone(),
                );
            }
        }

        Ok(DFA {
            dead: new_dead,
            start: new_start,
            accept: new_accept,
            storage: new_storage,
        })
    }

    /// Construct a new DFA that accepts the regular language that is the
    /// union of the regular languages represented by this DFA and
    /// another DFA.
    pub fn union<S2, S3>(&self, other: &DFA<A, S2>) -> Result<DFA<A, S3>, Error>
    where
        S::State: Hash,
        S2: DFAStorage<A>,
        S2::State: Ord + Hash,
        S3: DFAStorage<A>,
        S3::State: Ord,
        A: PartialEq + Clone,
        A::Symbol: Clone,
    {
        let (new_start, new_dead, new_storage, state_mapping) =
            cross_product_construction_without_accept_states::<S, S2, S3, A>(self, other)?;

        let other_all_states = other.storage.all_states();

        // The union accept states are all states where either `left_state` or
        // `right_state` was an accepting state.
        let accept_pairs =
            CartesianProductIter::new(self.storage.all_states().into_iter(), &other_all_states)
                .filter(|(left_state, right_state)| {
                    self.accept.contains(&left_state) || other.accept.contains(&right_state)
                });

        let mut new_accept: VecSet<S3::State> = VecSet::new();
        for (left_state, right_state) in accept_pairs {
            new_accept.insert(
                state_mapping
                    .get(&(left_state, right_state.clone()))
                    .ok_or(Error::StateNotFound)?
                    .clone(),
            );
        }

        Ok(DFA {
            dead: new_dead,
            start: new_start,
            accept: new_accept,
            storage: new_storage,
        })
    }

    /// Construct a new DFA that accepts the regular language that is the
    /// difference between regular languages represented by this DFA and
    /// another DFA.
    pub fn difference<S2, S3>(&self, other: &DFA<A, S2>) -> Result<DFA<A, S3>, Error>
    where
        S::State: Hash,
        S2: DFAStorage<A>,
        S2::State: Ord + Hash,
        S3: DFAStorage<A>,
        S3::State: Ord,
        A: PartialEq + Clone,
        A::Symbol: Clone,
    {
        let (new_start, new_dead, new_storage, state_mapping) =
            cross_product_construction_without_accept_states::<S, S2, S3, A>(self, other)?;

        let other_all_states = other.storage.all_states();

        // The difference accept states are all states where either `left_state` was an
        // accepting state and `right_state` was not.
        let accept_pairs =
            CartesianProductIter::new(self.storage.all_states().into_iter(), &other_all_states)
                .filter(|(left_state, right_state)| {
                    self.accept.contains(&left_state) && !other.accept.contains(&right_state)
                });

        let mut new_accept: VecSet<S3::State> = VecSet::new();
        for (left_state, right_state) in accept_pairs {
            new_accept.insert(
                state_mapping
                    .get(&(left_state, right_state.clone()))
                    .ok_or(Error::StateNotFound)?
                    .clone(),
            );
        }

        Ok(DFA {
            dead: new_dead,
            start: new_start,
            accept: new_accept,
            storage: new_storage,
        })
    }

    /// Construct a new DFA that accepts the regular language that is the
    /// complement of the regular language represented by this DFA.
    pub fn complement(&self) -> Self {
        let mut complement_dfa = DFA::clone(&self);

        // Swap the set of accepting and non-accepting states to get a DFA that accepts
        // the language complement of of the original DFA.
        let mut new_accept = self.storage.all_states();
        new_accept.retain(|s| !self.accept.contains(&s));
        complement_dfa.accept = new_accept.into();

        complement_dfa
    }
}
