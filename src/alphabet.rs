use crate::util::{Bounded, Range, Step};
use core::{
    hash::{BuildHasher, Hash},
    marker::PhantomData,
    ops,
};
use std::{
    collections::{BTreeSet, HashSet},
    vec::IntoIter,
};

/// A set of symbols.
pub trait Alphabet {
    /// The type of elements in this set.
    type Symbol;

    /// An iterator over all values in this alphabet.
    // TODO(GAT usage): if GAT's ever arrive, this could be
    // generic over the lifetime of the values origin.
    type ValueIter: Iterator<Item = Self::Symbol>;

    /// Return an iterator over all value in this alphabet.
    fn values(&self) -> Self::ValueIter;
    /// Return `true` if the given symbol is a member of this alphabet.
    fn contains(&self, sym: &Self::Symbol) -> bool;
    /// Optionally return the number of elements in this alphabet. Return None
    /// if the size is unbounded or would overflow a `usize` value.
    fn num_values(&self) -> Option<usize>;
}

impl<V, S> Alphabet for HashSet<V, S>
where
    V: Hash + Eq + Clone,
    S: BuildHasher,
{
    type Symbol = V;
    type ValueIter = IntoIter<V>;

    fn values(&self) -> Self::ValueIter {
        self.iter().map(V::clone).collect::<Vec<_>>().into_iter()
    }

    fn contains(&self, sym: &Self::Symbol) -> bool {
        HashSet::contains(self, sym)
    }

    fn num_values(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<V, S> Alphabet for &HashSet<V, S>
where
    V: Hash + Eq + Clone,
    S: BuildHasher,
{
    type Symbol = V;
    type ValueIter = IntoIter<V>;

    fn values(&self) -> Self::ValueIter {
        self.iter().map(V::clone).collect::<Vec<_>>().into_iter()
    }

    fn contains(&self, sym: &Self::Symbol) -> bool {
        HashSet::contains(self, sym)
    }

    fn num_values(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<V> Alphabet for BTreeSet<V>
where
    V: Ord + Clone,
{
    type Symbol = V;
    type ValueIter = IntoIter<V>;

    fn values(&self) -> Self::ValueIter {
        self.iter().map(V::clone).collect::<Vec<_>>().into_iter()
    }

    fn contains(&self, sym: &Self::Symbol) -> bool {
        BTreeSet::contains(self, &sym)
    }

    fn num_values(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<V> Alphabet for &BTreeSet<V>
where
    V: Ord + Clone,
{
    type Symbol = V;
    type ValueIter = IntoIter<V>;

    fn values(&self) -> Self::ValueIter {
        self.iter().map(V::clone).collect::<Vec<_>>().into_iter()
    }

    fn contains(&self, sym: &Self::Symbol) -> bool {
        BTreeSet::contains(self, &sym)
    }

    fn num_values(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<V> Alphabet for Range<V>
where
    V: Step,
{
    type Symbol = V;
    type ValueIter = Self;

    fn values(&self) -> Self::ValueIter {
        self.clone()
    }

    fn contains(&self, sym: &Self::Symbol) -> bool {
        match self {
            Range::NonEmpty { start, end } => start <= sym && sym <= end,
            Range::Empty => false,
        }
    }

    fn num_values(&self) -> Option<usize> {
        match self {
            Range::NonEmpty { start, end } => Step::steps_between(start, end),
            Range::Empty => Some(0),
        }
    }
}

/// Coversion into an alphabet.
pub trait IntoAlphabet {
    /// The type of symbols in the alphabet.
    type Symbol;
    /// The type of alphabet we are turning this into.
    type IntoAlpha: Alphabet<Symbol = Self::Symbol>;

    /// Create an alphabet from this value.
    fn into_alphabet(self) -> Self::IntoAlpha;
}

impl<A> IntoAlphabet for A
where
    A: Alphabet,
{
    type IntoAlpha = A;
    type Symbol = A::Symbol;

    fn into_alphabet(self) -> Self::IntoAlpha {
        self
    }
}

/// An object which can be turned into an alphabet which has all valid instances
/// of `Sym` as members of the alphabet.
pub struct Full<Sym: Step>(PhantomData<Sym>);

impl<Sym> IntoAlphabet for Full<Sym>
where
    Sym: Step + Bounded,
{
    type IntoAlpha = Range<Sym>;
    type Symbol = Sym;

    fn into_alphabet(self) -> Self::IntoAlpha {
        Range::NonEmpty {
            start: <Sym as Bounded>::MIN,
            end: <Sym as Bounded>::MAX,
        }
    }
}

impl<Sym> IntoAlphabet for ops::Range<Sym>
where
    Sym: Step,
{
    type IntoAlpha = Range<Sym>;
    type Symbol = Sym;

    fn into_alphabet(self) -> Self::IntoAlpha {
        Range::NonEmpty {
            start: self.start,
            end: self.end.predecessor(),
        }
    }
}

impl<Sym> IntoAlphabet for ops::RangeInclusive<Sym>
where
    Sym: Step,
{
    type IntoAlpha = Range<Sym>;
    type Symbol = Sym;

    fn into_alphabet(self) -> Self::IntoAlpha {
        Range::NonEmpty {
            start: self.start().clone(),
            end: self.end().clone(),
        }
    }
}

impl<Sym> IntoAlphabet for ops::RangeFrom<Sym>
where
    Sym: Step + Bounded,
{
    type IntoAlpha = Range<Sym>;
    type Symbol = Sym;

    fn into_alphabet(self) -> Self::IntoAlpha {
        Range::NonEmpty {
            start: self.start,
            end: <Sym as Bounded>::MAX,
        }
    }
}

impl<Sym> IntoAlphabet for ops::RangeTo<Sym>
where
    Sym: Step + Bounded,
{
    type IntoAlpha = Range<Sym>;
    type Symbol = Sym;

    fn into_alphabet(self) -> Self::IntoAlpha {
        Range::NonEmpty {
            start: <Sym as Bounded>::MIN,
            end: self.end.predecessor(),
        }
    }
}

impl<Sym> IntoAlphabet for ops::RangeToInclusive<Sym>
where
    Sym: Step + Bounded,
{
    type IntoAlpha = Range<Sym>;
    type Symbol = Sym;

    fn into_alphabet(self) -> Self::IntoAlpha {
        Range::NonEmpty {
            start: <Sym as Bounded>::MIN,
            end: self.end,
        }
    }
}
