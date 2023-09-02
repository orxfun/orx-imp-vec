use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<'a, T, P> ImpVec<T, P>
where
    P: PinnedVec<T> + 'a,
{
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
            let imp: ImpVec<_, _> = pinned_vec.into();

            let new_node = |data| N { data, next: None };

            let mut refs = vec![];
            for i in 0..1000 {
                let rf = imp.push_get_ref(new_node(i));
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
}
