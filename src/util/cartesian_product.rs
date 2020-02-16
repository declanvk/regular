use core::iter::{repeat, FusedIterator, Repeat, Zip};

/////////////////////////////
//    Cartesian Product    //
/////////////////////////////

pub struct CartesianProductIter<'a, IA, SB>
where
    IA: Iterator,
    IA::Item: Clone,
    &'a SB: IntoIterator,
{
    iter_a: IA,
    iter_b_producer: &'a SB,
    current_it: Option<Zip<Repeat<IA::Item>, <&'a SB as IntoIterator>::IntoIter>>,
}

impl<'a, IA, SB> CartesianProductIter<'a, IA, SB>
where
    IA: Iterator,
    IA::Item: Clone,
    &'a SB: IntoIterator,
{
    pub fn new(iter_a: IA, iter_b_producer: &'a SB) -> Self {
        CartesianProductIter {
            iter_a,
            iter_b_producer,
            current_it: None,
        }
    }
}

impl<'a, IA, SB> Clone for CartesianProductIter<'a, IA, SB>
where
    IA: Iterator + Clone,
    IA::Item: Clone,
    &'a SB: IntoIterator,
    <&'a SB as IntoIterator>::IntoIter: Clone,
{
    fn clone(&self) -> Self {
        CartesianProductIter {
            iter_a: self.iter_a.clone(),
            iter_b_producer: self.iter_b_producer,
            current_it: self.current_it.clone(),
        }
    }
}

impl<'a, IA, SB> Iterator for CartesianProductIter<'a, IA, SB>
where
    IA: Iterator,
    IA::Item: Clone,
    &'a SB: IntoIterator,
{
    type Item = (IA::Item, <&'a SB as IntoIterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current_it.as_mut() {
            if let Some(next) = current.next() {
                Some(next)
            } else {
                let mut new_it = repeat(self.iter_a.next()?).zip(self.iter_b_producer.into_iter());
                let next_item = new_it.next();

                *current = new_it;

                next_item
            }
        } else {
            let mut new_it = repeat(self.iter_a.next()?).zip(self.iter_b_producer.into_iter());
            let next_item = new_it.next();

            self.current_it = Some(new_it);

            next_item
        }
    }
}

impl<'a, IA, SB> FusedIterator for CartesianProductIter<'a, IA, SB>
where
    IA: Iterator + FusedIterator,
    IA::Item: Clone,
    &'a SB: IntoIterator,
    <&'a SB as IntoIterator>::IntoIter: FusedIterator,
{
}
