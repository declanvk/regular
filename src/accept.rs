/// Trait for objects that can decide whether or not a string belongs to some
/// language that is represented by the object.
pub trait Accept {
    /// The type of symbols in the strings.
    type Symbol;

    /// Return `true` if the given string is member of the language represented
    /// by this object.
    fn accept<I: IntoIterator<Item = Self::Symbol>>(self, string: I) -> bool;
}

/// Extension to the `Iterator` trait to test whether the contents of the
/// iterator would be accepted by the acceptor.
pub trait IterExt: Iterator {
    /// Return `true` if the string given by this iterator is a member of the
    /// language represented by the given object.
    fn is_accepted<M: Accept<Symbol = Self::Item>>(self, state_machine: M) -> bool
    where
        Self: Sized,
    {
        state_machine.accept(self)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;
    use core::marker::PhantomData;

    #[test]
    fn even_acceptor() {
        #[derive(Debug, Default)]
        struct AcceptEven<T>(PhantomData<T>);

        impl<T> Accept for AcceptEven<T> {
            type Symbol = T;

            fn accept<I: IntoIterator<Item = Self::Symbol>>(self, string: I) -> bool {
                string.into_iter().count() % 2 == 0
            }
        }

        assert!((0..=9).is_accepted(AcceptEven::default()));
        assert!(!(0..9).is_accepted(AcceptEven::default()));
    }

    #[test]
    fn accept_contains_element() {
        #[derive(Debug, Default)]
        struct Contains<T>(T);

        impl<T> Accept for Contains<T>
        where
            T: PartialEq,
        {
            type Symbol = T;

            fn accept<I: IntoIterator<Item = Self::Symbol>>(self, string: I) -> bool {
                for sym in string {
                    if sym == self.0 {
                        return true;
                    }
                }

                false
            }
        }

        assert!((0..=9).is_accepted(Contains(5)));
        assert!(!(0..=9).is_accepted(Contains(20)));
    }
}
