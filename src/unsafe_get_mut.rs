use crate::ImpVec;
use orx_split_vec::SplitVecGrowth;

impl<T, G> ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Returns a mutable reference to the element at the given `index`,
    /// returns None if the index is out of bounds.
    ///
    /// Unlike standard vector or `SplitVec`, this operation does **not**
    /// require a mut reference.
    ///
    /// # Safety
    ///
    /// The following are the reasons why the access is safe:
    ///
    /// * Due to the guarantees of the underlying `SplitVec`, pushed elements
    /// are pinned to their memory location unless a method requiring a mut
    /// reference to the vector is called. These are the methods that would
    /// lead to changing positions of existing elements such as `insert`, `remove`,
    /// `pop` or `clear`. On the other hand, `push`ing to or `extend`ing the
    /// imp-vec does not affect already added elements' memory locations.
    /// Therefore, whenever, we access an element with the given `index`,
    /// we are sure that we are accessing the correct element.
    /// Further, we are sure that the obtained mutable reference will be valid
    /// and targeting the correct data.
    ///
    /// * Mutation of the element is handled over an internal `RefCell`
    /// which would provide the guarantees that there will only be one
    /// mutable borrow at a time; the code will panic otherwise.
    ///
    /// The method is still marked unsafe due to the following reasons:
    ///
    /// * It makes it easy to have cyclic references which is often useful
    /// in cyclic data structures. However, the implementer needs to be
    /// careful for certain errors.
    /// Default implementations of traits such as `Debug` or `PartialEq` could
    /// easily lead to stackoverflows. See `ImpNode` struct which is a wrapper
    /// around the data to be stored in an imp-vec while avoiding such problems.
    ///
    /// * It is possible to hold an immutable reference to, say, the i-th
    /// element of the vector. Assume that the value of the element is 'x' at the
    /// time the reference is created. At the same time, it is also possible to get
    /// a mutable reference to the i-th element and mutate its value to 'y'.
    /// Finally, we can dereference the prior immutable reference just to read the
    /// data as 'y'. This is confusing, and hence, scope of these mutations should
    /// be kept limited; ideally, only while building a data structure which requires
    /// this feature. On the other hand, it is safe to dereference the prior
    /// immutable reference, the reference cannot be invalid due to the guarantees
    /// discussed above. A similar example is demonstrated below.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let vec = ImpVec::with_linear_growth(4);
    ///
    /// for i in 0..21 {
    ///     vec.push(i);
    /// }
    /// let immut_ref_7 = &vec[7];
    /// assert_eq!(7, *immut_ref_7);
    ///
    /// for i in 0..21 {
    ///     *unsafe{ vec.get_mut(i) }.unwrap() *= 100;
    /// }
    ///
    /// for i in 0..21 {
    ///     assert_eq!(100 * i, vec[i]);
    /// }
    /// assert_eq!(700, *immut_ref_7);
    /// ```
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut T> {
        self.get_fragment_and_inner_indices(index).map(|(f, i)| {
            let split_vec = unsafe { &mut *self.as_mut_ptr() };
            let fragments = unsafe { split_vec.fragments_mut() };
            &mut fragments[f][i]
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_all_growth_types, ImpVec};
    use orx_split_vec::SplitVecGrowth;

    #[test]
    fn get_mut() {
        fn test<G: SplitVecGrowth<usize>>(vec: ImpVec<usize, G>) {
            for i in 0..462 {
                vec.push(i);
                assert_eq!(None, unsafe { vec.get_mut(i + 1) });
            }

            let immut_refs: Vec<_> = vec.into_iter().collect();
            let mut mut_refs: Vec<_> = (0..vec.len())
                .map(|i| unsafe { vec.get_mut(i) }.expect("in-bounds"))
                .collect();

            for i in 0..462 {
                assert_eq!(i, vec[i]);
                assert_eq!(i, *immut_refs[i]);

                *mut_refs[i] *= 100;

                assert_eq!(i * 100, *immut_refs[i]);
                assert_eq!(i * 100, *mut_refs[i]);
            }

            for i in 0..462 {
                assert_eq!(i * 100, vec[i]);
            }
        }
        test_all_growth_types!(test);
    }
}
