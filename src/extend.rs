use crate::ImpVec;

impl<T: Clone> ImpVec<T> {
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

impl<'a, T: Clone + 'a> ImpVec<T> {
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
