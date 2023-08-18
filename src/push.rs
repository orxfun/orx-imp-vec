use crate::ImpVec;

impl<T> ImpVec<T> {
    /// Appends an element to the back of a collection.
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
        let split_vec = unsafe { &mut *self.as_mut_ptr() };
        let fragments = unsafe { split_vec.fragments_mut() };
        let f = fragments.len() - 1;
        if fragments[f].has_capacity_for_one() {
            fragments[f].push(value);
        } else {
            self.add_fragment_with_first_value(value);
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
    /// use orx_split_vec::FragmentGrowth;
    /// use orx_imp_vec::ImpVec;
    ///
    /// let vec = ImpVec::with_growth(FragmentGrowth::constant(4));
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
    /// use orx_imp_vec::ImpVec;
    ///
    /// let mut vec = ImpVec::default(); // mut required for the `insert`
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
        let split_vec = unsafe { &mut *self.as_mut_ptr() };
        let fragments = unsafe { split_vec.fragments_mut() };
        let mut f = fragments.len() - 1;
        if fragments[f].has_capacity_for_one() {
            fragments[f].push(value);
        } else {
            self.add_fragment_with_first_value(value);
            f += 1;
        }
        &fragments[f][fragments[f].len() - 1]
    }
}
