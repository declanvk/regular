use crate::util::FixedBitSet;
use core::{
    cmp::Ordering,
    ops::{Bound, RangeBounds},
    ptr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetPartitions {
    set_count: usize,
    elements: Box<[usize]>,
    marked: FixedBitSet,
    set_idx: Box<[usize]>,
    set_first_idx: Vec<usize>,
    set_last_idx: Vec<usize>,
    set_mid_idx: Vec<usize>,
}

pub type SetRepr = usize;

impl SetPartitions {
    #[allow(dead_code)]
    pub fn new(max: usize) -> (SetRepr, Self) {
        let num_elements = max + 1;
        let partitions = SetPartitions {
            elements: (0..=max).collect(),
            set_count: 1,
            marked: FixedBitSet::new(num_elements),
            set_idx: vec![0_usize; num_elements].into_boxed_slice(),
            set_first_idx: vec![0],
            set_last_idx: vec![num_elements],
            set_mid_idx: vec![0],
        };

        let default_set = 0;

        (default_set, partitions)
    }

    #[allow(dead_code)]
    pub fn num_partitions(&self) -> usize {
        self.set_count
    }

    #[allow(dead_code)]
    pub fn size(&self, set: SetRepr) -> usize {
        self.set_last_idx[set] - self.set_first_idx[set]
    }

    pub fn set(&self, item: usize) -> SetRepr {
        self.set_idx[item]
    }

    #[allow(dead_code)]
    pub fn mark(&mut self, item: usize) {
        let set = self.set_idx[item];
        let mid = self.set_mid_idx[set];
        let last = self.set_last_idx[set];

        if !self.marked.contains(item) {
            let loc = self.elements[mid..last].binary_search(&item).unwrap() + mid;
            // overwrite location of item
            copy_within(&mut self.elements, mid..loc, mid + 1);
            self.elements[mid] = item;
            self.set_mid_idx[set] = mid + 1;
            self.marked.set(item);
        } else {
        }
    }

    #[allow(dead_code)]
    pub fn split(&mut self, set: SetRepr) -> Option<SetRepr> {
        let first_orig = self.set_first_idx[set];
        let mid_orig = self.set_mid_idx[set];

        if mid_orig == self.set_last_idx[set] {
            self.set_mid_idx[set] = first_orig;
        }

        if first_orig == self.set_mid_idx[set] {
            None
        } else {
            self.set_count += 1;
            let new_set = self.set_count - 1;

            self.set_first_idx.push(first_orig);
            self.set_mid_idx.push(first_orig);
            self.set_last_idx.push(mid_orig);

            self.set_first_idx[set] = mid_orig;

            self.elements[self.set_first_idx[new_set]..self.set_last_idx[new_set]].sort_unstable();

            for loc in self.set_first_idx[new_set]..self.set_last_idx[new_set] {
                let elem_val = self.elements[loc];
                self.marked.clear(elem_val);
                self.set_idx[elem_val] = new_set;
            }

            Some(new_set)
        }
    }

    #[allow(dead_code)]
    pub fn set_iter(&self, set: SetRepr) -> SetPartitionIter {
        let (marked, unmarked) = (self.marked_slice(set), self.unmarked_slice(set));
        SetPartitionIter { marked, unmarked }
    }

    #[allow(dead_code)]
    pub fn no_marks(&self, set: SetRepr) -> bool {
        self.set_mid_idx[set] == self.set_first_idx[set]
    }

    #[allow(dead_code)]
    pub fn marked_slice(&self, set: SetRepr) -> &[usize] {
        &self.elements[self.set_first_idx[set]..self.set_mid_idx[set]]
    }

    #[allow(dead_code)]
    pub fn unmarked_slice(&self, set: SetRepr) -> &[usize] {
        &self.elements[self.set_mid_idx[set]..self.set_last_idx[set]]
    }
}

#[derive(Debug, Clone)]
pub struct SetPartitionIter<'a> {
    marked: &'a [usize],
    unmarked: &'a [usize],
}

impl<'a> Iterator for SetPartitionIter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first_marked, rest_marked)) = self.marked.split_first() {
            if let Some((first_unmarked, rest_unmarked)) = self.unmarked.split_first() {
                match first_marked.cmp(first_unmarked) {
                    Ordering::Equal => {
                        self.marked = rest_marked;
                        self.unmarked = rest_unmarked;

                        Some(first_marked)
                    }
                    Ordering::Greater => {
                        self.unmarked = rest_unmarked;

                        Some(first_unmarked)
                    }
                    Ordering::Less => {
                        self.marked = rest_marked;
                        Some(first_marked)
                    }
                }
            } else {
                self.marked = rest_marked;

                Some(first_marked)
            }
        } else if let Some((first, rest)) = self.unmarked.split_first() {
            self.unmarked = rest;

            Some(first)
        } else {
            None
        }
    }
}

pub fn copy_within<T, R: RangeBounds<usize>>(slice: &mut [T], src: R, dest: usize) {
    let src_start = match src.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n.checked_add(1).unwrap(),
        Bound::Unbounded => 0,
    };
    let src_end = match src.end_bound() {
        Bound::Included(&n) => n.checked_add(1).unwrap(),
        Bound::Excluded(&n) => n,
        Bound::Unbounded => slice.len(),
    };
    assert!(src_start <= src_end, "src end is before src start");
    assert!(src_end <= slice.len(), "src is out of bounds");
    let count = src_end - src_start;
    assert!(dest <= slice.len() - count, "dest is out of bounds");
    unsafe {
        ptr::copy(
            slice.get_unchecked(src_start),
            slice.get_unchecked_mut(dest),
            count,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_set_partition() {
        let (default_set, partitions): (usize, SetPartitions) = SetPartitions::new(7);

        assert_eq!(partitions.size(default_set), 8);
        assert_eq!(partitions.set(0), default_set);
    }

    #[test]
    fn create_single_unit_partition() {
        let (default_set, mut partitions): (usize, SetPartitions) = SetPartitions::new(7);

        assert!(partitions.no_marks(default_set));
        partitions.mark(3);
        assert!(!partitions.no_marks(default_set));
        assert_eq!(partitions.set(3), default_set);

        let new_set = partitions.split(default_set).unwrap();
        assert!(partitions.no_marks(new_set));
        assert_eq!(partitions.set(3), new_set);
        assert_eq!(partitions.size(new_set), 1);
        assert_eq!(partitions.size(default_set), 7);
    }

    #[test]
    fn create_single_large_partition() {
        let (default_set, mut partitions): (usize, SetPartitions) = SetPartitions::new(7);

        partitions.mark(3);
        partitions.mark(7);
        partitions.mark(2);
        partitions.mark(0);

        assert_eq!(partitions.size(default_set), 8);
        let new_set = partitions.split(default_set).unwrap();
        assert!(partitions.no_marks(new_set));
        assert!(partitions.no_marks(default_set));
        assert_eq!(partitions.size(default_set), 4);
        assert_eq!(partitions.size(new_set), 4);
    }

    #[test]
    fn create_multiple_large_partitions() {
        let (set_a, mut partitions): (usize, SetPartitions) = SetPartitions::new(8);

        partitions.mark(3);
        partitions.mark(7);
        partitions.mark(2);

        assert!(!partitions.no_marks(set_a));
        assert_eq!(partitions.size(set_a), 9);

        let set_b = partitions.split(set_a).unwrap();

        assert!(partitions.no_marks(set_b));
        assert!(partitions.no_marks(set_a));
        assert_eq!(partitions.size(set_a), 6);
        assert_eq!(partitions.size(set_b), 3);

        partitions.mark(1);
        partitions.mark(4);
        partitions.mark(8);

        let set_c = partitions.split(set_a).unwrap();

        assert!(partitions.no_marks(set_c));
        assert!(partitions.no_marks(set_a));
        assert_eq!(partitions.size(set_a), 3);
        assert_eq!(partitions.size(set_c), 3);
    }

    #[test]
    fn create_nested_partitions() {
        let (set_a, mut partitions): (usize, SetPartitions) = SetPartitions::new(9);

        partitions.mark(3);
        partitions.mark(7);
        partitions.mark(1);
        partitions.mark(5);
        partitions.mark(9);

        assert!(!partitions.no_marks(set_a));
        assert_eq!(partitions.size(set_a), 10);
        let set_b = partitions.split(set_a).unwrap();
        assert_eq!(partitions.size(set_a), 5);
        assert_eq!(partitions.size(set_b), 5);

        partitions.mark(1);
        partitions.mark(9);

        let set_c = partitions.split(set_b).unwrap();
        assert_eq!(partitions.size(set_a), 5);
        assert_eq!(partitions.size(set_b), 3);
        assert_eq!(partitions.size(set_c), 2);
    }
}
