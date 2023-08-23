use orx_split_vec::SplitVecGrowth;

use crate::ImpVec;

impl<T: Clone, G> ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Clones and appends all elements in a slice to the vec.
    ///
    /// Iterates over the slice `other`, clones each element, and then appends
    /// it to this vector. The `other` slice is traversed in-order.
    ///
    /// Unlike `std::vec::Vec` or `orx_split_vec::SplitVec`;
    /// push operation for `ImpVec` does **not** require a mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let vec = ImpVec::default();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend_from_slice(&[4, 5, 6, 7]);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn extend_from_slice(&self, other: &[T]) {
        let split_vec = unsafe { &mut *self.as_mut_ptr() };
        let fragments = unsafe { split_vec.fragments_mut() };

        let mut slice = other;
        while !slice.is_empty() {
            if !self.has_capacity_for_one() {
                self.add_fragment();
            }
            let f = fragments.len() - 1;

            let last = &mut fragments[f];
            let available = last.room();

            if available < slice.len() {
                last.extend_from_slice(&slice[0..available]);
                slice = &slice[available..];
                self.add_fragment();
            } else {
                last.extend_from_slice(slice);
                break;
            }
        }
    }
}

impl<'a, T: Clone + 'a, G> ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Clones and appends all elements in the iterator to the vec.
    ///
    /// Iterates over the `iter`, clones each element, and then appends
    /// it to this vector.
    ///
    /// Unlike `std::vec::Vec` or `orx_split_vec::SplitVec`;
    /// extend operation for `ImpVec` does **not** require a mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let vec = ImpVec::default();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.push(3);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.extend(&[4, 5, 6, 7]);
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    ///
    /// let sec_vec = ImpVec::default();
    /// sec_vec.extend(vec.into_iter());
    /// assert_eq!(sec_vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn extend<I: IntoIterator<Item = &'a T>>(&self, iter: I) {
        for x in iter {
            self.push(x.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_all_growth_types, ImpVec};
    use orx_split_vec::SplitVecGrowth;

    #[test]
    fn extend_from_slice() {
        fn test<G: SplitVecGrowth<usize>>(vec: ImpVec<usize, G>) {
            let mut refs = Vec::with_capacity(4062);
            let mut addr = Vec::with_capacity(4062);

            vec.extend_from_slice(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                let refi = &vec[i];
                refs.push(refi);
                addr.push(refi as *const usize);
            }

            vec.extend_from_slice(&(42..63).collect::<Vec<_>>());
            for i in 42..63 {
                let refi = &vec[i];
                refs.push(refi);
                addr.push(refi as *const usize);
            }

            vec.extend_from_slice(&(63..100).collect::<Vec<_>>());
            for i in 63..100 {
                let refi = &vec[i];
                refs.push(refi);
                addr.push(refi as *const usize);
            }

            assert_eq!(100, vec.len());
            for i in 0..100 {
                assert_eq!(i, vec[i]);

                assert_eq!(i, *refs[i]);

                let curr_addr = &vec[i] as *const usize;
                assert_eq!(curr_addr, addr[i]);

                assert_eq!(i, unsafe { *addr[i] });
            }
        }
        test_all_growth_types!(test);
    }

    #[test]
    fn push_get_ref2() {
        fn test<G: SplitVecGrowth<usize>>(vec: ImpVec<usize, G>) {
            let mut refs = Vec::with_capacity(4062);
            let mut addr = Vec::with_capacity(4062);

            vec.extend(&(0..42).collect::<Vec<_>>());
            for i in 0..42 {
                let refi = &vec[i];
                refs.push(refi);
                addr.push(refi as *const usize);
            }

            vec.extend(&(42..63).collect::<Vec<_>>());
            for i in 42..63 {
                let refi = &vec[i];
                refs.push(refi);
                addr.push(refi as *const usize);
            }

            vec.extend(&(53..90).map(|i| i + 10).collect::<Vec<_>>());
            for i in 63..100 {
                let refi = &vec[i];
                refs.push(refi);
                addr.push(refi as *const usize);
            }

            assert_eq!(100, vec.len());
            for i in 0..100 {
                assert_eq!(i, vec[i]);

                assert_eq!(i, *refs[i]);

                let curr_addr = &vec[i] as *const usize;
                assert_eq!(curr_addr, addr[i]);

                assert_eq!(i, unsafe { *addr[i] });
            }
        }
        test_all_growth_types!(test);
    }
}
