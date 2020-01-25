use core::{iter::Peekable, marker::PhantomData};

/////////////////////////////
//         Joining         //
/////////////////////////////

pub struct Join<T, IL, IR, F>
where
    IL: Iterator<Item = T>,
    IR: Iterator<Item = T>,
{
    left: Peekable<IL>,
    right: Peekable<IR>,
    f: F,
    _items: PhantomData<T>,
}

impl<T, IL, IR, F> Join<T, IL, IR, F>
where
    IL: Iterator<Item = T> + Clone,
    IR: Iterator<Item = T> + Clone,
    F: FnMut(&mut Peekable<IL>, &mut Peekable<IR>) -> Option<T>,
{
    pub fn new<L, R>(left: L, right: R, join: F) -> Self
    where
        L: IntoIterator<IntoIter = IL, Item = T>,
        R: IntoIterator<IntoIter = IR, Item = T>,
    {
        Join {
            left: left.into_iter().peekable(),
            right: right.into_iter().peekable(),
            f: join,
            _items: PhantomData,
        }
    }
}

impl<IL, IR, T, F> Iterator for Join<T, IL, IR, F>
where
    IL: Iterator<Item = T> + Clone,
    IR: Iterator<Item = T> + Clone,
    F: FnMut(&mut Peekable<IL>, &mut Peekable<IR>) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.left, &mut self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_flop_join_iter() {
        let mut toggle = true;

        let lhs = 0..10;
        let rhs = 10..20;

        let toggled: Vec<_> = Join::new(lhs.into_iter(), rhs.into_iter(), |left, right| {
            let next = if toggle { left.next() } else { right.next() };

            toggle = !toggle;

            next
        })
        .take(9)
        .collect();

        assert_eq!(toggled, &[0, 10, 1, 11, 2, 12, 3, 13, 4]);
    }
}
