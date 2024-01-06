use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<'a, T, P> ImpVec<T, P>
where
    P: PinnedVec<T> + 'a,
{
    /// Replaces the data at the `at`-th position of the vector with the `new_value`;
    /// and returns the old data at this position.
    ///
    /// Does nothing and returns None if `at` is out of bounds.
    ///
    /// # Safety
    ///
    /// This method is safe due to the following:
    ///
    /// * acts only if `at` is within the storage owned by the vector.
    /// * all references to the element at the `at`-th position are still valid since the length of the vector does not change.
    /// * the underlying data at this position change; however, this is done with a `&mut self` reference.
    #[inline(always)]
    pub fn replace_at(&mut self, at: usize, new_value: T) -> Option<T> {
        unsafe { self.get_mut(at) }.map(|old_value| std::mem::replace(old_value, new_value))
    }

    /// Appends an element to the back of a collection and returns a reference to it.
    ///
    /// Unlike `std::vec::Vec` or `orx_split_vec::SplitVec`;
    /// push operation for `ImpVec` does **not** require a mutable reference (see Safety section).
    ///
    /// This is a crucial method for building self-referential-collections which is the underlying motivation of an `ImpVec`.
    /// See the Motivation section for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::prelude::{ImpVec, PinnedVec};
    ///
    /// let mut vec: ImpVec<_> = ImpVec::new();
    ///
    /// let ref1 = unsafe { vec.push_get_ref(1) };
    /// let ref_elem_addr = ref1 as *const i32;
    ///
    /// vec.push(2);
    /// vec.push(3);
    /// let ref4 = unsafe { vec.push_get_ref(4) };
    ///
    /// // capacity is expaneded here from 4 to 8; however, in chunks;
    /// // therefore, data is not moved around and the references remain valid.
    /// let ref5 = unsafe { vec.push_get_ref(5) };
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
    /// let ref1 = unsafe { vec.push_get_ref(1) };
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
    ///
    /// # Safety
    ///
    /// This method is **unsafe** due to the broken lifetime relation of the returned reference from this vector.
    /// In brief, the returned reference can outlive this vector which is undefined behavior (UB).
    /// The following example demonstrates that this UB can be experienced.
    ///
    /// ```rust
    /// use orx_imp_vec::prelude::*;
    ///
    /// let mut imp: ImpVec<_> = ImpVec::new();
    /// let first = unsafe { imp.push_get_ref('a') };
    ///
    /// assert_eq!(&'a', &imp[0]);
    /// assert_eq!(&'a', first);
    ///
    /// drop(imp);
    ///
    /// // assert_eq!(&'a', &imp[0]); // this should NOT compile, and correctly does NOT compile
    ///
    /// // the following should also NOT compile; however, it does!
    /// assert_eq!(&'a', first);
    /// ```
    ///
    /// Due to the demonstrated problem, `push_get_ref` method is `unsafe` and should be used carefully.
    ///
    /// In particular, caller we must take the responsibility to make sure that the returned reference does not outlive the vector.
    /// The wrapper structures or methods can safely be used once this guarantee is provided.
    ///
    /// # Motivation
    ///
    /// The reason this method exists is to narrow down the unsafe code and complexity.
    /// To be precise, the entire complexity of building self-referential-collections can be narrowed to the limited usage of
    /// two main unsafe methods `push_get_ref` and [`crate::ImpVec::move_get_ref`].
    ///
    /// Consider for instance two doubly-linked list implementations `std::collections::LinkedList` and `orx_linked_list::LinkedList`.
    ///
    /// `std::collections::LinkedList` is a relatively low level implementation with heavy use of raw pointers.
    /// The file [https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html](https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html)
    /// contains `unsafe` keyword used more than 60 times.
    /// The unsafe code blocks contain reads/writes to memory through raw pointers which is dangerous and allows much more unsafety than required for defining a linked list.
    ///
    /// A higher level implementation can be found here [https://crates.io/crates/orx-linked-list](https://crates.io/crates/orx-linked-list)
    /// which is built on an underlying `ImpVec` storage.
    /// Complete repository contains the `unsafe` keyword seven times corresponding to repeated usage of only three methods:
    ///
    /// * `ImpVec::push_get_ref`
    /// * `ImpVec::move_get_ref`
    /// * `ImpVec::unsafe_truncate` which actually is a deref method from `PinnedVec::unsafe_truncate`
    ///
    /// In brief, a full-fetched doubly-linked-list can be built in rust:
    /// * with thin references
    /// * without smart pointers
    /// * without any raw pointers
    /// * by using only three unsafe methods which are specialized for building self referential collections
    ///
    /// Furthermore, it performs significantly faster proving the performance benefit by cache locality which can be attained by
    /// putting nodes close to each other within an `ImpVec`.
    pub unsafe fn push_get_ref<'b>(&'b mut self, value: T) -> &'a T
    where
        P: 'a,
        'a: 'b,
    {
        let vec = self.pinned_vec();
        vec.push(value);
        vec.get_unchecked(vec.len() - 1)
    }

    /// Performs the following:
    ///
    /// * copies the data at `source_idx` onto the element at `destination_idx`;
    /// * replaces the data at the `source_idx` with the `fill_source_with`;
    /// * returns references to the new values of the elements at (source_idx, destination_idx).
    ///
    /// Does nothing and returns None if any of `source_idx` and `destination_idx` is out of bounds.
    ///
    /// Note that after the move operation
    ///
    /// * value at the `destination_idx` will be equal to the prior value at the `source_idx`, and
    /// * value at the `source_idx` will be equal to `fill_source_with`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let mut imp: ImpVec<_> = ['a', 'b', 'c'].into_iter().collect();
    ///
    /// assert_eq!(&['a', 'b', 'c'], &imp);
    ///
    /// let (ref0, ref1) = unsafe { imp.move_get_ref(0, 1, 'x') }.unwrap();
    ///
    /// assert_eq!(&imp[0], &'x');
    /// assert_eq!(&imp[1], &'a');
    ///
    /// assert_eq!(ref0, &'x');
    /// assert_eq!(ref1, &'a');
    /// ```
    ///
    /// # Safety
    ///
    /// This method is safe due to the following:
    ///
    /// * acts only if `source_idx` and `destination_idx` are within the storage owned by the vector.
    /// * all references to the element at the `source_idx`-th position and `destination_idx`-th position are still valid since the length of the vector does not change.
    /// * the underlying data at these positions change; however, this is done with a `&mut self` reference.
    ///
    /// On the other hand, this method is **unsafe** due to the broken lifetime relation of the returned references from this vector.
    /// In brief, the returned references can outlive this vector which is undefined behavior (UB).
    /// The following example demonstrates that this UB can be experienced.
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let mut imp: ImpVec<_> = ['a', 'b', 'c'].into_iter().collect();
    ///
    /// assert_eq!(&['a', 'b', 'c'], &imp);
    ///
    /// let (ref0, ref1) = unsafe { imp.move_get_ref(0, 1, 'x') }.unwrap();
    ///
    /// assert_eq!(&imp[0], &'x');
    /// assert_eq!(&imp[1], &'a');
    ///
    /// assert_eq!(ref0, &'x');
    /// assert_eq!(ref1, &'a');
    ///
    /// drop(imp);
    ///
    /// // following two lines should NOT compile, and correctly do NOT compile
    /// // assert_eq!(&imp[0], &'x');
    /// // assert_eq!(&imp[1], &'a');
    ///
    /// // the following lines should also NOT compile; however, they do!
    /// assert_eq!(ref0, &'x');
    /// assert_eq!(ref1, &'a');
    /// ```
    ///
    /// Due to the demonstrated problem, `move_get_ref` method is `unsafe` and should be used carefully.
    ///
    /// In particular, caller we must take the responsibility to make sure that the returned references does not outlive the vector.
    /// The wrapper structures or methods can safely be used once this guarantee is provided.
    ///
    /// # Motivation
    ///
    /// The reason this method exists is to narrow down the unsafe code and complexity.
    /// To be precise, the entire complexity of building self-referential-collections can be narrowed to the limited usage of
    /// two main unsafe methods `move_get_ref` and [`crate::ImpVec::push_get_ref`].
    ///
    /// Consider for instance two doubly-linked list implementations `std::collections::LinkedList` and `orx_linked_list::LinkedList`.
    ///
    /// `std::collections::LinkedList` is a relatively low level implementation with heavy use of raw pointers.
    /// The file [https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html](https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html)
    /// contains `unsafe` keyword used more than 60 times.
    /// The unsafe code blocks contain reads/writes to memory through raw pointers which is dangerous and allows much more unsafety than required for defining a linked list.
    ///
    /// A higher level implementation can be found here [https://crates.io/crates/orx-linked-list](https://crates.io/crates/orx-linked-list)
    /// which is built on an underlying `ImpVec` storage.
    /// Complete repository contains the `unsafe` keyword seven times corresponding to repeated usage of only three methods:
    ///
    /// * `ImpVec::push_get_ref`
    /// * `ImpVec::move_get_ref`
    /// * `ImpVec::unsafe_truncate` which actually is a deref method from `PinnedVec::unsafe_truncate`
    ///
    /// In brief, a full-fetched doubly-linked-list can be built in rust:
    /// * with thin references
    /// * without smart pointers
    /// * without any raw pointers
    /// * by using only three unsafe methods which are specialized for building self referential collections
    ///
    /// Furthermore, it performs significantly faster proving the performance benefit by cache locality which can be attained by
    /// putting nodes close to each other within an `ImpVec`.
    pub unsafe fn move_get_ref<'b>(
        &mut self,
        source_idx: usize,
        destination_idx: usize,
        fill_source_with: T,
    ) -> Option<(&'b T, &'b T)>
    where
        'a: 'b,
    {
        let swap = self.try_mem_swap(source_idx, destination_idx);
        swap.map(|_| {
            _ = self.replace_at(source_idx, fill_source_with);

            (
                self.get_ref(source_idx).expect("issome"),
                self.get_ref(destination_idx).expect("issome"),
            )
        })
    }

    // helpers - crate
    /// Returns a mutable reference to the item at the `index`-th position of the vector;
    /// returns None if index is out of bounds.
    ///
    /// The main purpose of this method is to be able to build vectors
    /// elements of which reference other elements,
    /// while these references lead to cyclic relations.
    ///
    /// # Safety
    ///
    /// This method allows to mutate an existing element of the vector
    /// with an immutable reference.
    /// For obvious reasons, this operation is not safe.
    /// Therefore, it is important that this method is used in limited
    /// scopes, where the caller is able to guarantee the safety
    /// of the call.
    pub(crate) unsafe fn get_mut<'b>(&self, index: usize) -> Option<&'a mut T>
    where
        'a: 'b,
    {
        let data = self.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            pinned_vec.get_mut(index)
        }
    }

    /// Returns a reference to the item at the `index`-th position of the vector;
    /// returns None if index is out of bounds.
    ///
    /// The main purpose of this method is to be able to build vectors
    /// elements of which reference other elements,
    /// while these references lead to cyclic relations.
    ///
    /// # Safety
    ///
    /// This method allows to mutate an existing element of the vector
    /// with an immutable reference.
    /// For obvious reasons, this operation is not safe.
    /// Therefore, it is important that this method is used in limited
    /// scopes, where the caller is able to guarantee the safety
    /// of the call.
    /// See the `get_mut` examples related to safety.
    pub(crate) unsafe fn get_ref<'b>(&self, index: usize) -> Option<&'b T>
    where
        'a: 'b,
    {
        let data = self.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            pinned_vec.get(index)
        }
    }

    // helpers
    unsafe fn try_mem_swap(&self, first_idx: usize, second_idx: usize) -> Option<()> {
        match (self.get_mut(first_idx), self.get_mut(second_idx)) {
            (Some(x), Some(y)) => {
                std::mem::swap(x, y);
                Some(())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_pinned_types;

    #[test]
    fn get_mut() {
        struct N<'a> {
            data: usize,
            next: Option<&'a N<'a>>,
        }

        fn test<'a, P: PinnedVec<N<'a>> + 'a>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            let new_node = |data| N { data, next: None };

            let mut refs = vec![];
            for i in 0..1000 {
                let rf = unsafe { imp.push_get_ref(new_node(i)) };
                refs.push(Some(rf));
            }

            unsafe { imp.get_mut(999) }.expect("-").next = refs[0];
            for i in 0..999 {
                unsafe { imp.get_mut(i) }.expect("-").next = refs[i + 1];
            }

            for i in 0..999 {
                assert_eq!(i, imp[i].data);
                assert_eq!(Some(i + 1), imp[i].next.map(|x| x.data))
            }
            assert_eq!(999, imp[999].data);
            assert_eq!(Some(0), imp[999].next.map(|x| x.data))
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn push_get_ref() {
        fn test<P: PinnedVec<u32>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            let first = unsafe { imp.push_get_ref(0) };
            imp.push(1);
            imp.push(2);
            imp.push(3);

            assert_eq!(0, imp[0]);
            assert_eq!(&0, first);

            // push 800 more
            for i in 0..800 {
                imp.push(i)
            }

            assert_eq!(0, imp[0]);
            assert_eq!(&0, first);
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn move_get_ref() {
        fn test<P: PinnedVec<u32>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            imp.push(0);
            imp.push(1);
            imp.push(2);
            imp.push(3);

            let (x, y) = unsafe { imp.move_get_ref(0, 3, 10) }.expect("issome");

            assert_eq!(10, imp[0]);
            assert_eq!(0, imp[3]);

            assert_eq!(&10, x);
            assert_eq!(&0, y);

            assert!(unsafe { imp.move_get_ref(0, 4, 10) }.is_none());
            assert!(unsafe { imp.move_get_ref(4, 0, 10) }.is_none());
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn replace_at() {
        fn test<P: PinnedVec<u32>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            imp.push(0);
            imp.push(1);
            imp.push(2);
            imp.push(3);

            let old = imp.replace_at(0, 10);

            assert_eq!(Some(0), old);
            assert_eq!(10, imp[0]);
        }

        test_all_pinned_types!(test);
    }
}
