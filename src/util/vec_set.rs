use crate::util::join_iter::Join;
use core::{
    cmp::Ordering,
    iter::{FromIterator, Peekable},
    slice::{Iter, IterMut},
};
use std::vec::IntoIter;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct VecSet<A> {
    inner: Vec<A>,
}

fn is_sorted<'a, A: 'a + Ord>(iter: impl IntoIterator<Item = &'a A>) -> bool {
    let mut iter = iter.into_iter();
    let mut last = match iter.next() {
        Some(e) => e,
        None => return true,
    };

    for curr in iter {
        if last > curr {
            return false;
        }
        last = curr;
    }

    true
}

fn is_deduped<'a, A: 'a + Eq>(iter: impl IntoIterator<Item = &'a A>) -> bool {
    let mut iter = iter.into_iter();
    let mut last = match iter.next() {
        Some(e) => e,
        None => return true,
    };

    for curr in iter {
        if last == curr {
            return false;
        }
        last = curr;
    }

    true
}

impl<A> VecSet<A>
where
    A: Ord,
{
    pub fn new() -> Self {
        VecSet { inner: Vec::new() }
    }
}

impl<A> VecSet<A>
where
    A: Ord + Clone,
{
    pub fn from_slice(src: &[A]) -> Self {
        let mut inner: Vec<_> = src.into();

        inner.sort_unstable();
        inner.dedup();

        VecSet { inner }
    }

    pub unsafe fn from_slice_unchecked(src: &[A]) -> Self {
        debug_assert!(is_sorted(src.iter()));
        debug_assert!(is_deduped(src.iter()));

        let inner = src.into();

        VecSet { inner }
    }

    pub fn contains(&self, item: &A) -> bool {
        self.inner.binary_search(item).is_ok()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&A) -> bool,
    {
        self.inner.retain(f);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn insert(&mut self, item: A) -> bool {
        match self.inner.binary_search(&item) {
            Ok(_) => false,
            Err(idx) => {
                self.inner.insert(idx, item);
                true
            }
        }
    }

    pub fn remove(&mut self, item: &A) -> bool {
        match self.inner.binary_search(item) {
            Ok(idx) => {
                self.inner.remove(idx);
                true
            }
            Err(_idx) => false,
        }
    }

    pub fn intersection<'a>(&'a self, other: &'a VecSet<A>) -> impl Iterator<Item = &'a A> {
        Join::new(self, other, intersection_logic)
    }

    pub fn difference<'a>(&'a self, other: &'a VecSet<A>) -> impl Iterator<Item = &'a A> {
        Join::new(self, other, difference_logic)
    }

    pub fn union<'a>(&'a self, other: &'a VecSet<A>) -> impl Iterator<Item = &'a A> {
        Join::new(self, other, |left, right| {
            match cmp_opt(left.peek(), right.peek(), Ordering::Greater, Ordering::Less) {
                Ordering::Less => left.next(),
                Ordering::Equal => {
                    right.next();
                    left.next()
                }
                Ordering::Greater => right.next(),
            }
        })
    }

    pub fn symmetric_difference<'a>(&'a self, other: &'a VecSet<A>) -> impl Iterator<Item = &'a A> {
        Join::new(self, other, |left, right| loop {
            match cmp_opt(left.peek(), right.peek(), Ordering::Greater, Ordering::Less) {
                Ordering::Less => return left.next(),
                Ordering::Equal => {
                    left.next();
                    right.next();
                }
                Ordering::Greater => return right.next(),
            }
        })
    }

    pub fn iter(&self) -> Iter<A> {
        self.inner.iter()
    }
}

impl<A> VecSet<A> {
    pub fn as_slice(&self) -> &[A] {
        &self.inner
    }
}

impl<A: Ord + Clone> From<&[A]> for VecSet<A> {
    fn from(inner: &[A]) -> Self {
        let mut inner = inner.to_vec();
        inner.sort_unstable();

        VecSet { inner }
    }
}

impl<A: Ord> From<Vec<A>> for VecSet<A> {
    fn from(mut inner: Vec<A>) -> Self {
        inner.sort_unstable();

        VecSet { inner }
    }
}

impl<A> Extend<A> for VecSet<A>
where
    A: Ord + Eq,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.inner.extend(iter);
        self.inner.sort_unstable();
        self.inner.dedup();
    }
}

impl<A> FromIterator<A> for VecSet<A>
where
    A: Ord + Eq,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut inner: Vec<_> = iter.into_iter().collect();
        inner.sort_unstable();
        inner.dedup();

        VecSet { inner }
    }
}

impl<A> IntoIterator for VecSet<A> {
    type IntoIter = IntoIter<A>;
    type Item = A;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a, A> IntoIterator for &'a VecSet<A> {
    type IntoIter = Iter<'a, A>;
    type Item = &'a A;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a, A> IntoIterator for &'a mut VecSet<A> {
    type IntoIter = IterMut<'a, A>;
    type Item = &'a mut A;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl<A> Default for VecSet<A> {
    fn default() -> Self {
        VecSet {
            inner: Vec::default(),
        }
    }
}

fn cmp_opt<T: Ord>(x: Option<&T>, y: Option<&T>, short: Ordering, long: Ordering) -> Ordering {
    match (x, y) {
        (None, _) => short,
        (_, None) => long,
        (Some(x1), Some(y1)) => x1.cmp(y1),
    }
}

pub fn intersection_logic<'a, A: Ord>(
    left: &mut Peekable<Iter<'a, A>>,
    right: &mut Peekable<Iter<'a, A>>,
) -> Option<&'a A> {
    loop {
        match left.peek()?.cmp(right.peek()?) {
            Ordering::Equal => {
                left.next();
                return right.next();
            }
            Ordering::Less => left.next(),
            Ordering::Greater => right.next(),
        };
    }
}

pub fn difference_logic<'a, A: Ord>(
    left: &mut Peekable<Iter<'a, A>>,
    right: &mut Peekable<Iter<'a, A>>,
) -> Option<&'a A> {
    loop {
        match cmp_opt(left.peek(), right.peek(), Ordering::Less, Ordering::Less) {
            Ordering::Less => return left.next(),
            Ordering::Equal => {
                left.next();
                right.next();
            }
            Ordering::Greater => {
                right.next();
            }
        }
    }
}
