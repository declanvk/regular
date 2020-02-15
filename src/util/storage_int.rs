use core::{convert::TryInto, ops};

mod private {
    pub trait Sealed {}
}

pub trait StorageInt:
    private::Sealed
    + ops::Shl<usize, Output = Self>
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::Not<Output = Self>
    + PartialEq
    + Eq
    + Sized
    + Copy
    + Ord
{
    fn one() -> Self;
    fn zero() -> Self;
    fn leading_zeros(self) -> usize;

    fn from_usize(src: usize) -> Self;
    fn to_usize(self) -> usize;
}

macro_rules! impl_storage_int {
    ($($t:ty),*) => {
        $(
            impl private::Sealed for $t {}

            impl StorageInt for $t {
                fn one() -> Self {
                    1
                }

                fn zero() -> Self {
                    0
                }

                fn leading_zeros(self) -> usize {
                    Self::leading_zeros(self).try_into().unwrap()
                }

                fn from_usize(src: usize) -> Self {
                    src.try_into().unwrap()
                }

                fn to_usize(self) -> usize {
                    self.try_into().unwrap()
                }
            }
        )*
    };
}

impl_storage_int!(u8, u16, u32, u64, usize, u128);
