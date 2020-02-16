use core::{
    cmp,
    convert::TryFrom,
    mem::replace,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Range<Sym: Step> {
    NonEmpty {
        /// Start value, inclusive
        start: Sym,
        /// End value, inclusive
        end: Sym,
    },
    Empty,
}

impl<Sym> Range<Sym>
where
    Sym: Step,
{
    pub fn contains(&self, sym: Sym) -> bool {
        match self {
            Range::NonEmpty { start, end } => start <= &sym && &sym <= end,
            Range::Empty => false,
        }
    }
}

impl<Sym> Iterator for Range<Sym>
where
    Sym: Step,
{
    type Item = Sym;

    fn next(&mut self) -> Option<Self::Item> {
        let range = replace(self, Range::Empty);

        match range {
            Range::NonEmpty { start, end } if start != end => {
                if let Some(new_start) = start.forward(1) {
                    *self = Range::NonEmpty {
                        start: new_start,
                        end,
                    };
                }

                Some(start)
            }
            Range::NonEmpty { start, end: _ } => Some(start),
            Range::Empty => None,
        }
    }
}

pub trait Bounded: Copy {
    const MIN: Self;
    const MAX: Self;
}

/// TODO(replace copied code): This is a copy of the Step trait from https://github.com/rust-lang/rust/pull/62886
/// When the PR is merged or when the trait hits stable, replace this.
///
/// Objects that have a notion of *successor* and *predecessor*.
///
/// The *successor* operation moves towards values that compare greater.
/// The *predecessor* operation moves towards values that compare lesser.
///
/// # Safety
///
/// This trait is `unsafe` because its implementation must be correct for
/// the safety of `unsafe trait TrustedLen` implementations, and the results
/// of using this trait can be otherwise trusted by `unsafe` code.
pub unsafe trait Step: Clone + PartialOrd + Sized {
    /// Returns the number of *successor* steps needed to get from `start` to
    /// `end`.
    ///
    /// Returns `None` if that number would overflow `usize`
    /// (or is infinite, or if `end` would never be reached).
    ///
    /// # Invariants
    ///
    /// For any `a`, `b`, and `n`:
    ///
    /// * `steps_between(&a, &b) == Some(n)` if and only if `a.forward(n) ==
    ///   Some(b)`
    /// * `steps_between(&a, &b) == Some(n)` if and only if `b.backward(n) ==
    ///   Some(a)`
    /// * `steps_between(&a, &b) == Some(n)` only if `a <= b`
    ///   * Corrolary: `steps_between(&a, &b) == Some(0)` if and only if `a ==
    ///     b`
    ///   * Note that `a <= b` does _not_ imply `steps_between(&a, &b) != None`;
    ///     this is the case when it would take more than `usize::MAX` steps to
    ///     get to `b`
    /// * `steps_between(&a, &b) == None` if `a > b`
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    /// Returns the value that would be obtained by taking the *successor*
    /// of `self` `count` times.
    ///
    /// Returns `None` if this would overflow the range of values supported by
    /// `Self`.
    ///
    /// # Invariants
    ///
    /// For any `a`, `n`, and `m` where `n + m` does not overflow:
    ///
    /// * `a.forward(n).and_then(|x| x.forward(m)) == a.forward(n + m)`
    /// * `a.forward(n)` equals `Step::successor` applied to `a` `n` times
    ///   * Corollary: `a.forward(0) == Some(a)`
    /// * `a.forward(n).unwrap() >= a`
    fn forward(&self, count: usize) -> Option<Self>;

    /// Returns the *successor* of `self`.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this method is allowed to panic or wrap. Suggested behavior is
    /// to panic when debug assertions are enabled, and wrap otherwise.
    ///
    /// # Invariants
    ///
    /// For any `a` where `a.successor()` does not overflow:
    ///
    /// * `a == a.successor().predecessor()`
    /// * `a.successor() == a.forward(1).unwrap()`
    /// * `a.successor() >= a`
    #[inline]
    fn successor(&self) -> Self {
        self.forward(1).expect("overflow in `Step::successor`")
    }

    /// Returns the *successor* of `self`.
    ///
    /// If this would overflow the range of values supported by `Self`,
    /// this method is defined to return the input value instead.
    ///
    /// # Invariants
    ///
    /// For any `a` where `a.successor()` does not overflow:
    ///
    /// * `a == a.successor().predecessor()`
    /// * `a.successor() == a.forward(1).unwrap()`
    /// * `a.successor() >= a`
    ///
    /// For any `a` where `a.successor()` does overflow:
    ///
    /// * `a.successor() == a`
    #[inline]
    fn successor_saturating(&self) -> Self {
        self.forward(1).unwrap_or_else(|| self.clone())
    }

    /// Returns the *successor* of `self` without overflow.
    ///
    /// # Safety
    ///
    /// It is undefined behavior if this operation exceeds the range of
    /// values supported by `Self`. If you cannot guarantee that this
    /// will not overflow, use `forward` or `successor` instead.
    ///
    /// For any `a`, if there exists `b` such that `b > a`,
    /// it is safe to call `a.successor_unchecked()`.
    #[inline]
    unsafe fn successor_unchecked(&self) -> Self {
        self.successor()
    }

    /// Returns the value that would be obtained by taking the *predecessor*
    /// of `self` `count` times.
    ///
    /// Returns `None` if this would underflow the range of values supported by
    /// `Self`.
    ///
    /// # Invariants
    ///
    /// For any `a`, `n`, and `m` where `n + m` does not overflow:
    ///
    /// * `a.backward(n).and_then(|x| x.backward(m)) == a.backward(n + m)`
    /// * `a.backward(n)` equals `Step::predecessor` applied to `a` `n` times
    ///   * Corollary: `a.backward(0) == Some(a)`
    /// * `a.backward(n).unwrap() <= a`
    fn backward(&self, count: usize) -> Option<Self>;

    /// Returns the *predecessor* of `self`.
    ///
    /// If this would underflow the range of values supported by `Self`,
    /// this method is allowed to panic or wrap. Suggested behavior is
    /// to panic when debug assertions are enabled, and wrap otherwise.
    ///
    /// # Invariants
    ///
    /// For any `a` where `a.predecessor()` does not underflow:
    ///
    /// * `a == a.predecessor().successor()`
    /// * `a.predecessor() == a.backward(1).unwrap()`
    /// * `a.predecessor() <= a`
    #[inline]
    fn predecessor(&self) -> Self {
        self.backward(1).expect("underflow in `Step::predecessor`")
    }

    /// Returns the *predecessor* of `self`.
    ///
    /// If this would underflow the range of values supported by `Self`,
    /// this method is defined to return the input value instead.
    ///
    /// # Invariants
    ///
    /// For any `a` where `a.predecessor()` does not underflow:
    ///
    /// * `a == a.predecessor().successor()`
    /// * `a.predecessor() == a.backward(1).unwrap()`
    /// * `a.predecessor() <= a`
    ///
    /// For any `a` where `a.predecessor()` does underflow:
    ///
    /// * `a.predecessor() == a`
    #[inline]
    fn predecessor_saturating(&self) -> Self {
        self.backward(1).unwrap_or_else(|| self.clone())
    }

    /// Returns the *predecessor* of `self` without underflow.
    ///
    /// # Safety
    ///
    /// It is undefined behavior if this operation exceeds the range of
    /// values supported by `Self`. If you cannot guarantee that this
    /// will not underflow, use `backward` or `predecessor` instead.
    ///
    /// For any `a`, if there exists `b` such that `b < a`,
    /// it is safe to call `a.successor_unchecked()`.
    #[inline]
    unsafe fn predecessor_unchecked(&self) -> Self {
        self.predecessor()
    }
}

// These are still macro-generated because the integer literals resolve to
// different types.
macro_rules! step_identical_methods {
    () => {
        #[inline]
        fn successor(&self) -> Self {
            Add::add(*self, 1)
        }

        #[inline]
        fn successor_saturating(&self) -> Self {
            Self::saturating_add(*self, 1)
        }

        #[inline]
        fn predecessor(&self) -> Self {
            Sub::sub(*self, 1)
        }

        #[inline]
        fn predecessor_saturating(&self) -> Self {
            Self::saturating_sub(*self, 1)
        }
    };
}

macro_rules! step_integer_impls {
    (
        narrower than or same width as usize:
            $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
        wider than usize:
            $( [ $wider_unsigned:ident $wider_signed: ident ] ),+;
    ) => {
        $(
            unsafe impl Step for $narrower_unsigned {
                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        // This relies on $narrower_unsigned <= usize
                        Some((*end - *start) as usize)
                    } else {
                        None
                    }
                }

                #[inline]
                #[allow(unreachable_patterns)]
                fn forward(&self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n_converted) => self.checked_add(n_converted),
                        Err(_) => None,  // if n is out of range, `something_unsigned + n` is too
                    }
                }

                #[inline]
                #[allow(unreachable_patterns)]
                fn backward(&self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n_converted) => self.checked_sub(n_converted),
                        Err(_) => None,  // if n is out of range, `something_in_range - n` is too
                    }
                }

                step_identical_methods!();
            }

            unsafe impl Step for $narrower_signed {
                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        // This relies on $narrower_signed <= usize
                        //
                        // Casting to isize extends the width but preserves the sign.
                        // Use wrapping_sub in isize space and cast to usize
                        // to compute the difference that may not fit inside the range of isize.
                        Some((*end as isize).wrapping_sub(*start as isize) as usize)
                    } else {
                        None
                    }
                }

                #[inline]
                #[allow(unreachable_patterns)]
                fn forward(&self, n: usize) -> Option<Self> {
                    match <$narrower_unsigned>::try_from(n) {
                        Ok(n_unsigned) => {
                            // Wrapping in unsigned space handles cases like
                            // `-120_i8.forward(200) == Some(80_i8)`,
                            // even though 200_usize is out of range for i8.
                            let self_unsigned = *self as $narrower_unsigned;
                            let wrapped = self_unsigned.wrapping_add(n_unsigned) as Self;
                            if wrapped >= *self {
                                Some(wrapped)
                            } else {
                                None  // Addition overflowed
                            }
                        }
                        // If n is out of range of e.g. u8,
                        // then it is bigger than the entire range for i8 is wide
                        // so `any_i8 + n` would overflow i8.
                        Err(_) => None,
                    }
                }

                #[inline]
                #[allow(unreachable_patterns)]
                fn backward(&self, n: usize) -> Option<Self> {
                    match <$narrower_unsigned>::try_from(n) {
                        Ok(n_unsigned) => {
                            // Wrapping in unsigned space handles cases like
                            // `-120_i8.forward(200) == Some(80_i8)`,
                            // even though 200_usize is out of range for i8.
                            let self_unsigned = *self as $narrower_unsigned;
                            let wrapped = self_unsigned.wrapping_sub(n_unsigned) as Self;
                            if wrapped <= *self {
                                Some(wrapped)
                            } else {
                                None  // Subtraction underflowed
                            }
                        }
                        // If n is out of range of e.g. u8,
                        // then it is bigger than the entire range for i8 is wide
                        // so `any_i8 - n` would underflow i8.
                        Err(_) => None,
                    }
                }

                step_identical_methods!();
            }
        )+

        $(
            unsafe impl Step for $wider_unsigned {
                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        usize::try_from(*end - *start).ok()
                    } else {
                        None
                    }
                }

                #[inline]
                fn forward(&self, n: usize) -> Option<Self> {
                    self.checked_add(n as Self)
                }

                #[inline]
                fn backward(&self, n: usize) -> Option<Self> {
                    self.checked_sub(n as Self)
                }

                step_identical_methods!();
            }

            unsafe impl Step for $wider_signed {
                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        match end.checked_sub(*start) {
                            Some(diff) => usize::try_from(diff).ok(),
                            // If the difference is too big for e.g. i128,
                            // it’s also gonna be too big for usize with fewer bits.
                            None => None
                        }
                    } else {
                        None
                    }
                }

                #[inline]
                fn forward(&self, n: usize) -> Option<Self> {
                    self.checked_add(n as Self)
                }

                #[inline]
                fn backward(&self, n: usize) -> Option<Self> {
                    self.checked_sub(n as Self)
                }

                step_identical_methods!();
            }
        )+
    }
}

#[cfg(target_pointer_width = "64")]
step_integer_impls! {
    narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
    wider than usize: [u128 i128];
}

#[cfg(target_pointer_width = "32")]
step_integer_impls! {
    narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [usize isize];
    wider than usize: [u64 i64], [u128 i128];
}

#[cfg(target_pointer_width = "16")]
step_integer_impls! {
    narrower than or same width as usize: [u8 i8], [u16 i16], [usize isize];
    wider than usize: [u32 i32], [u64 i64], [u128 i128];
}

#[inline]
fn char_successor_saturating(src: char) -> char {
    const CHAR_MAX: u32 = core::char::MAX as u32;
    let src_int: u32 = src.into();

    let out_int = match src_int {
        CHAR_MAX => src_int, // skip (src > char::MAX)
        0xD7FF => 0xE000,    // skip (src >= 0xD800 && src <= 0xDFFF)
        _ => src_int + 1,
    };

    // should always succeed
    char::try_from(out_int).unwrap()
}

#[inline]
fn char_predecessor_saturating(src: char) -> char {
    let src_int: u32 = src.into();

    let out_int = match src_int {
        0 => 0,           // skip (src > char::MAX)
        0xE000 => 0xD7FF, // skip (src >= 0xD800 && src <= 0xDFFF)
        _ => src_int - 1,
    };

    // should always succeed
    char::try_from(out_int).unwrap()
}

#[inline]
fn char_successor_panic(src: char) -> char {
    const CHAR_MAX: u32 = char::MAX as u32;
    let src_int: u32 = src.into();

    let out_int = match src_int {
        CHAR_MAX => panic!("'char' overflow on successor!"), // skip (src > char::MAX)
        0xD7FF => 0xE000,                                    /* skip (src >= 0xD800 && src <= */
        // 0xDFFF)
        _ => src_int + 1,
    };

    // should always succeed
    char::try_from(out_int).unwrap()
}

#[inline]
fn char_predecessor_panic(src: char) -> char {
    let src_int: u32 = src.into();

    let out_int = match src_int {
        0 => panic!("'char' underflow on predecessor!"), // skip (src > char::MAX)
        0xE000 => 0xD7FF,                                // skip (src >= 0xD800 && src <= 0xDFFF)
        _ => src_int - 1,
    };

    // should always succeed
    char::try_from(out_int).unwrap()
}

#[inline]
fn char_forward(src: char, steps: usize) -> Option<char> {
    let src_int: u32 = src.into();
    let out_int = u32::checked_add(src_int, u32::try_from(steps).ok()?)?;

    let adjusted_out = if out_int >= 0xD800 && src_int < 0xD800 {
        let overlap = cmp::min(out_int, 0xDFFF) - 0xD800 + 1;

        u32::checked_add(overlap, 0xDFFF)
    } else {
        Some(out_int)
    };

    char::try_from(adjusted_out?).ok()
}

#[inline]
fn char_backward(src: char, steps: usize) -> Option<char> {
    let src_int: u32 = src.into();
    let out_int = u32::checked_sub(src_int, u32::try_from(steps).ok()?)?;

    let adjusted_out = if out_int <= 0xDFFF && src_int > 0xDFFF {
        let overlap = 0xDFFF - cmp::max(out_int, 0xD800) + 1;

        u32::checked_sub(0xD800, overlap)
    } else {
        Some(out_int)
    };

    char::try_from(adjusted_out?).ok()
}

#[cfg(target_pointer_width = "16")]
unsafe impl Step for char {
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        let (start, end): (u32, u32) = (start.into(), end.into());

        if start > end {
            None
        } else if start < 0xD800 && end > 0xDFFF {
            usize::try_from((end - start) - (0xDFFF - 0xD800 + 1)).ok()
        } else {
            usize::try_from(end - start).ok()
        }
    }

    #[inline]
    #[allow(unreachable_patterns)]
    fn forward(&self, n: usize) -> Option<Self> {
        char_forward(*self, n)
    }

    #[inline]
    #[allow(unreachable_patterns)]
    fn backward(&self, n: usize) -> Option<Self> {
        char_backward(*self, n)
    }

    #[inline]
    fn successor(&self) -> Self {
        char_successor_panic(*self)
    }

    #[inline]
    fn successor_saturating(&self) -> Self {
        char_successor_saturating(*self)
    }

    #[inline]
    fn predecessor(&self) -> Self {
        char_predecessor_panic(*self)
    }

    #[inline]
    fn predecessor_saturating(&self) -> Self {
        char_predecessor_saturating(*self)
    }
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
unsafe impl Step for char {
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        let (start, end) = (u32::from(*start), u32::from(*end));

        if start > end {
            None
        } else if start < 0xD800 && end > 0xDFFF {
            Some(((end - start) - (0xDFFF - 0xD800 + 1)) as usize)
        } else {
            Some((end - start) as usize)
        }
    }

    #[inline]
    fn forward(&self, n: usize) -> Option<Self> {
        char_forward(*self, n)
    }

    #[inline]
    fn backward(&self, n: usize) -> Option<Self> {
        char_backward(*self, n)
    }

    #[inline]
    fn successor(&self) -> Self {
        char_successor_panic(*self)
    }

    #[inline]
    fn successor_saturating(&self) -> Self {
        char_successor_saturating(*self)
    }

    #[inline]
    fn predecessor(&self) -> Self {
        char_predecessor_panic(*self)
    }

    #[inline]
    fn predecessor_saturating(&self) -> Self {
        char_predecessor_saturating(*self)
    }
}

macro_rules! impl_bounded_for_integer {
    ($($t:ty|$p:ident),*) => {
        $(
            impl Bounded for $t {
                const MAX: Self = core::$p::MAX;
                const MIN: Self = core::$p::MIN;
            }
        )*
    };
}

impl_bounded_for_integer!(
    u8 | u8,
    i8 | i8,
    u16 | u16,
    i16 | i16,
    u32 | u32,
    i32 | i32,
    u64 | u64,
    i64 | i64,
    u128 | u128,
    i128 | i128
);

impl Bounded for char {
    const MAX: Self = core::char::MAX;
    const MIN: Self = 0 as char;
}

impl Bounded for bool {
    const MAX: Self = true;
    const MIN: Self = false;
}

impl Bounded for () {
    const MAX: Self = ();
    const MIN: Self = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iter::successors;

    #[test]
    fn char_steps_between_sanity() {
        assert_eq!(Step::steps_between(&'a', &'z'), Some(25));
        assert_eq!(Step::steps_between(&'\u{D7FF}', &'\u{E000}'), Some(1));
        assert_eq!(Step::steps_between(&'z', &'a'), None);
    }

    #[test]
    fn char_successor_and_predecessor() {
        assert_eq!(Step::predecessor(&Step::successor(&'\u{D7FF}')), '\u{D7FF}');
        assert_eq!(Step::successor(&'a'), 'b');
        assert_eq!(Step::predecessor(&'b'), 'a');

        assert_eq!(Step::successor(&'\u{D7FF}'), '\u{E000}');
        assert_eq!(Step::predecessor(&'\u{E000}'), '\u{D7FF}');

        assert_eq!(Step::successor_saturating(&'a'), 'b');
        assert_eq!(
            Step::successor_saturating(&core::char::MAX),
            core::char::MAX
        );
        assert_eq!(Step::predecessor_saturating(&'z'), 'y');
        assert_eq!(Step::predecessor_saturating(&(0 as char)), (0 as char));
    }

    #[test]
    fn char_forward_backward_and_steps_between_agree() {
        let num_valid_chars = successors(Some(0 as char), |n| Step::forward(n, 1)).count();
        let num_valid_chars_back =
            successors(Some(core::char::MAX), |n| Step::backward(n, 1)).count();

        assert_eq!(num_valid_chars, 1112063 + 1);
        assert_eq!(num_valid_chars_back, 1112063 + 1);
        assert_eq!(
            Step::steps_between(&'\u{0}', &core::char::MAX),
            Some(1112063)
        );
    }
}
