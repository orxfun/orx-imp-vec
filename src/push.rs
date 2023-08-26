use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P> ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    /// Appends an element to the back of a collection.
    ///
    /// Unlike `std::vec::Vec` or `orx_split_vec::SplitVec`;
    /// push operation for `ImpVec` does **not** require a mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::prelude::*;
    ///
    /// let vec: ImpVec<_, _> = FixedVec::new(10).into();
    /// vec.push(1);
    /// vec.push(2);
    ///
    /// // since push does not require a mut reference,
    /// // it is legal to hold on to other immutable references
    /// // while pushing elements.
    /// let ref_elem = &vec[1];
    /// let ref_elem_addr = ref_elem as *const i32;
    /// assert_eq!(2, *ref_elem);
    ///
    /// vec.push(3);
    /// vec.push(4);
    /// vec.push(5);
    ///
    /// assert_eq!(2, *ref_elem);
    /// assert_eq!(vec, [1, 2, 3, 4, 5]);
    ///
    /// let ref_elem_addr_after_growth = &vec[1] as *const i32;
    /// assert_eq!(ref_elem_addr, ref_elem_addr_after_growth);
    /// ```
    pub fn push(&self, value: T) {
        let data = self.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            pinned_vec.push(value);
        }
    }

    /// Appends an element to the back of a collection and returns a reference to it.
    ///
    /// The reference will always be valid unless the collection is mutated;
    /// note that methods that grows the vector do **not** require a mutable reference,
    /// such as, `push`, `push_get_ref`, `extend` or `extend_from_slice` methods.
    ///
    /// # Examples
    ///
    /// Hold on to valid references while pushing new items,
    /// as long as the collection is not mutated with methods such as `insert`, `remove` or `pop`.
    ///
    /// ```
    /// use orx_imp_vec::prelude::*;
    ///
    /// let vec: ImpVec<_, _> = FixedVec::new(10).into();
    /// let ref1 = vec.push_get_ref(1);
    /// let ref_elem_addr = ref1 as *const i32;
    ///
    /// vec.push(2);
    /// vec.push(3);
    /// let ref4 = vec.push_get_ref(4);
    ///
    /// // capacity is expaneded here from 4 to 8; however, in chunks;
    /// // therefore, data is not moved around and the references remain valid.
    /// let ref5 = vec.push_get_ref(5);
    ///
    ///
    /// assert_eq!(ref1, &1);
    /// assert_eq!(ref4, &4);
    /// assert_eq!(ref5, &5);
    /// assert_eq!(vec, [1, 2, 3, 4, 5]);
    ///
    /// let ref_elem_addr_after_growth = &vec[0] as *const i32;
    /// assert_eq!(ref_elem_addr, ref_elem_addr_after_growth);
    /// ```
    ///
    /// As you may see below, any mutable method that can possibly invalidate the references
    /// are not allowed.
    ///
    /// ```
    /// use orx_imp_vec::prelude::*;
    ///
    /// let mut vec: ImpVec<_, _> = SplitVec::with_linear_growth(10).into(); // mut required for the `insert`
    /// let ref1 = vec.push_get_ref(1);
    /// vec.push(2);
    /// vec.push(3);
    ///
    /// assert_eq!(ref1, &1);
    /// assert_eq!(vec, [1, 2, 3]);
    ///
    /// vec.insert(0, 42);
    /// assert_eq!(vec, [42, 1, 2, 3]);
    ///
    /// // below line does not compile as the 'insert' call breaks reference 'ref1'
    /// // let value1 = *ref1;
    /// ```
    pub fn push_get_ref(&self, value: T) -> &T {
        let data = self.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            pinned_vec.push(value);
            pinned_vec.get_unchecked(pinned_vec.len() - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn push() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            let mut initial_refs = vec![];
            for i in 0..1000 {
                imp.push(i * 10);
                initial_refs.push(&imp[i] as *const usize);
            }

            let expected_vals: Vec<_> = (0..1000).map(|i| i * 10).collect();
            assert_eq!(expected_vals, imp);

            let mut final_refs = vec![];
            for i in 0..1000 {
                final_refs.push(&imp[i] as *const usize);
            }
            assert_eq!(initial_refs, final_refs);
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn push_get_ref() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            let mut initial_refs = vec![];

            let first_ref = imp.push_get_ref(0);
            initial_refs.push(first_ref as *const usize);

            for i in 1..1000 {
                initial_refs.push(imp.push_get_ref(i * 10) as *const usize);
            }

            let expected_vals: Vec<_> = (0..1000).map(|i| i * 10).collect();
            assert_eq!(expected_vals, imp);

            let mut final_refs = vec![];
            for i in 0..1000 {
                final_refs.push(&imp[i] as *const usize);
            }
            assert_eq!(initial_refs, final_refs);

            assert_eq!(0, *first_ref);
        }

        test_all_growth_types!(test);
    }
}
