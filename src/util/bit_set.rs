use crate::util::StorageInt;
use core::{fmt, mem::size_of};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixedBitSet<B = u32> {
    size: usize,
    inner: Box<[B]>,
}

fn bit_width<B>() -> usize {
    8 * size_of::<B>()
}

fn blocks_for_bits<B>(bits: usize) -> usize {
    let block_size = bit_width::<B>();
    if bits % block_size == 0 {
        bits / block_size
    } else {
        bits / block_size + 1
    }
}

impl<B> FixedBitSet<B>
where
    B: StorageInt,
{
    pub fn new(size: usize) -> Self {
        FixedBitSet {
            size,
            inner: vec![B::zero(); blocks_for_bits::<B>(size)].into_boxed_slice(),
        }
    }

    pub fn block_len(&self) -> usize {
        self.inner.len()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn contains(&self, value: &usize) -> bool {
        self.get(*value).unwrap_or(false)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner
            .iter()
            .zip(other.inner.iter())
            .all(|(sub, sup)| sub <= sup)
    }

    pub fn get(&self, elem: usize) -> Option<bool> {
        let block_size = bit_width::<B>();
        let block = elem / block_size;
        let bit = elem % block_size;

        self.inner.get(block).map(|&block_value| {
            let masked: B = block_value & (B::one() << bit);
            masked != B::zero()
        })
    }

    pub fn set(&mut self, elem: usize) -> bool {
        let block_size = bit_width::<B>();
        let block = elem / block_size;
        let bit = elem % block_size;

        if let Some(block) = self.inner.get_mut(block) {
            *block = *block | (B::one() << bit);

            true
        } else {
            false
        }
    }

    pub fn clear(&mut self, elem: usize) -> bool {
        let block_size = bit_width::<B>();
        let block = elem / block_size;
        let bit = elem % block_size;

        if let Some(block) = self.inner.get_mut(block) {
            *block = *block & !(B::one() << bit);

            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        let block_width = bit_width::<B>();
        self.inner
            .iter()
            .enumerate()
            .flat_map(move |(block_idx, block)| {
                BlockIter(*block).map(move |bit_idx| block_idx * block_width + bit_idx)
            })
    }
}

impl<B> fmt::Binary for FixedBitSet<B>
where
    B: fmt::Binary,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (idx, block) in self.inner.iter().enumerate() {
            if idx != self.inner.len() - 1 {
                write!(f, "{:b}, ", block)?;
            } else {
                write!(f, "{:b}", block)?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone)]
pub struct BlockIter<B>(B);

impl<B> Iterator for BlockIter<B>
where
    B: StorageInt,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == B::zero() {
            None
        } else {
            let leading_0s = self.0.leading_zeros() as usize;

            self.0 = self.0 & !(B::one() << leading_0s);

            Some(leading_0s)
        }
    }
}
