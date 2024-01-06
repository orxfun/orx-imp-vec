use orx_fixed_vec::FixedVec;
use orx_pinned_vec::PinnedVec;
use orx_split_vec::{Growth, SplitVec};
use std::{cell::RefCell, marker::PhantomData};

/// A data structure to enable conveniently building performant self referential collections such as trees, graphs and linked lists.
///
/// `ImpVec` 👿 is a wrapper around a [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) which guarantees that memory location of its elements do not change unless elements are removed from the vector. In addition to dereferencing the `PinnedVec` methods, `ImpVec` adds the following methods which are specialized for defining and building **self referential collections** using plain `&` references:
///
/// ```rust ignore
/// fn set_next(&mut self, element: &T, next: Option<&'a T>)
/// fn set_prev(&mut self, element: &T, prev: Option<&'a T>)
/// fn replace_at(&mut self, at: usize, new_value: T) -> Option<T>
/// unsafe fn push_get_ref(&self, value: T) -> &T
/// unsafe fn move_get_ref(&mut self, source_idx: usize, destination_idx: usize, fill_source_with: T) -> Option<(&T, &T)>
/// ```
pub struct ImpVec<T, P = SplitVec<T>>
where
    P: PinnedVec<T>,
{
    cell: RefCell<P>,
    phantom: PhantomData<T>,
}

impl<T, P: PinnedVec<T> + Default> Default for ImpVec<T, P> {
    fn default() -> Self {
        Self {
            cell: RefCell::new(P::default()),
            phantom: Default::default(),
        }
    }
}

impl<T, P: PinnedVec<T>> ImpVec<T, P> {
    /// Creates a new empty vector.
    pub fn new() -> Self
    where
        P: Default,
    {
        Default::default()
    }

    /// Converts the ImpVec into underlying PinnedVec.
    ///
    /// This operation corresponds to moving the pinned vec out of a ref cell; and hence, is not expensive.
    /// Likewise, the pinned vec can then be converted back into an imp vec by wrapping it up by a ref cell under the hood.
    ///
    /// Converting an ImpVec to PinnedVec is useful for the following reasons:
    ///
    /// * It reduces one level of abstraction by getting rid of the ref cell, and hence,
    /// achieving SplitVec or FixedVec (same as std vec) performance depending on the type of the pinned vec.
    /// * It means that immutable pushes are not allowed anymore.
    ///     * In a vector holding inter-element references, this means that such references will not be added or removed.
    ///     * In other words, it safely ensures that the building of the structure is completed.
    ///
    /// # Examples
    ///
    /// You may see and example back-and-forth conversions between a PinnedVec.
    ///
    /// *Complete type annotations are not needed byt added to make the conversions explicitly visible.*
    ///
    /// ```
    /// use orx_imp_vec::prelude::*;
    ///
    /// let mut imp: ImpVec<char, SplitVec<char, Doubling>> = SplitVec::with_doubling_growth().into();
    /// for _ in 0..3 {
    ///     imp.push('s');
    /// }
    /// assert_eq!(imp, vec!['s', 's', 's']);
    ///
    /// let mut split: SplitVec<char, Doubling> = imp.into_pinned();
    /// assert_eq!(split, vec!['s', 's', 's']);
    ///
    /// let mut imp_again: ImpVec<_, _> = split.into();
    /// imp_again.push('s');
    /// assert_eq!(imp_again, vec!['s', 's', 's', 's']);
    ///
    /// let split_back = imp_again.into_pinned();
    /// assert_eq!(split_back, vec!['s', 's', 's', 's']);
    /// ```
    pub fn into_pinned(self) -> P {
        self.cell.into_inner()
    }

    // helpers
    pub(crate) fn new_from(split_vec: RefCell<P>) -> Self {
        Self {
            cell: split_vec,
            phantom: Default::default(),
        }
    }
    pub(crate) fn as_mut_ptr(&self) -> *mut P {
        self.cell.as_ptr()
    }
    pub(crate) fn cell(&self) -> &RefCell<P> {
        &self.cell
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub(crate) unsafe fn pinned_vec<'a>(&self) -> &'a mut P {
        let data = self.as_mut_ptr();
        unsafe { &mut *data }
    }
}

// From<ImpVec>
impl<T, P: PinnedVec<T>> From<ImpVec<T, P>> for RefCell<P> {
    fn from(value: ImpVec<T, P>) -> Self {
        value.cell
    }
}
impl<T> From<ImpVec<T, FixedVec<T>>> for FixedVec<T> {
    fn from(value: ImpVec<T, FixedVec<T>>) -> Self {
        value.into_pinned()
    }
}

impl<T, G: Growth> From<ImpVec<T, SplitVec<T, G>>> for SplitVec<T, G> {
    fn from(value: ImpVec<T, SplitVec<T, G>>) -> Self {
        value.into_pinned()
    }
}
// Into<ImpVec>
impl<T, P: PinnedVec<T>> From<P> for ImpVec<T, P> {
    fn from(value: P) -> Self {
        Self::new_from(RefCell::new(value))
    }
}
impl<T, P: PinnedVec<T> + FromIterator<T>> FromIterator<T> for ImpVec<T, P> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        P::from_iter(iter).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;
    use crate::test_all_pinned_types;
    use std::cell::RefCell;

    #[test]
    fn as_mut_ptr() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();
            let ptr = imp.as_mut_ptr();
            for i in 0..1000 {
                imp.push(i);
            }

            assert_eq!(ptr, imp.as_mut_ptr());
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn default() {
        fn test<P: PinnedVec<char> + Default>() {
            let default_pin = P::default();
            let default_imp = ImpVec::<char>::default();
            let imp_from_default_pin: ImpVec<_, _> = default_pin.into();
            assert_eq!(default_imp, imp_from_default_pin);
        }

        test::<SplitVec<char, Linear>>();
        test::<SplitVec<char, Doubling>>();
    }

    #[test]
    fn into_pinned() {
        fn to_vec<P: PinnedVec<usize>>(pinned: &P) -> Vec<usize> {
            (0..pinned.len())
                .map(|i| *pinned.get(i).expect("is-some"))
                .collect()
        }
        fn test<P: PinnedVec<usize>>(mut pinned_vec: P) {
            for x in 0..451 {
                pinned_vec.push(x * 3 + 3);
            }
            let vec_from_pinned = to_vec(&pinned_vec);

            let mut imp: ImpVec<_, _> = pinned_vec.into();
            imp.push(42);
            _ = imp.pop();

            let pinned_back: RefCell<P> = imp.into();
            let pinned_back = pinned_back.into_inner();
            let vec_from_pinned_back = to_vec(&pinned_back);

            assert_eq!(vec_from_pinned, vec_from_pinned_back);
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn default_type_params() {
        let imp: ImpVec<char> = SplitVec::new().into();
        assert_eq!(SplitVec::new(), imp);

        let imp: ImpVec<char> = SplitVec::with_doubling_growth().into();
        assert_eq!(SplitVec::with_doubling_growth(), imp);

        let imp: ImpVec<char, FixedVec<char>> = FixedVec::new(10).into();
        assert_eq!(imp, FixedVec::new(42));

        let imp: ImpVec<char, SplitVec<char, Linear>> = SplitVec::with_linear_growth(2).into();
        assert_eq!(SplitVec::with_linear_growth(2), imp);

        let imp: ImpVec<char, SplitVec<char, Doubling>> = SplitVec::with_doubling_growth().into();
        assert_eq!(SplitVec::with_linear_growth(22), imp);
    }

    #[test]
    fn iter_iter_mut() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            for i in 0..100 {
                imp.push(i);
            }

            for x in imp.iter_mut() {
                *x += 42;
            }

            for (i, x) in imp.iter().enumerate() {
                assert_eq!(i + 42, *x);
            }

            for x in imp.iter_mut() {
                *x += 42;
            }

            for (i, x) in imp.iter().enumerate() {
                assert_eq!(i + 42 + 42, *x);
            }
        }

        test_all_pinned_types!(test);
    }

    // push
    #[test]
    fn push() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();
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

        test_all_pinned_types!(test);
    }

    #[test]
    fn push_get_ref() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();
            let mut initial_refs = vec![];

            let first_ref = unsafe { imp.push_get_ref(0) };
            initial_refs.push(first_ref as *const usize);

            for i in 1..1000 {
                initial_refs.push(unsafe { imp.push_get_ref(i * 10) } as *const usize);
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

        test_all_pinned_types!(test);
    }

    #[test]
    fn insert() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            for i in 0..1000 {
                imp.insert(0, i);
            }

            for i in 0..1000 {
                let val = 1000 - 1 - i;
                assert_eq!(val, imp[i]);
            }
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn remove() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            for i in 0..1000 {
                imp.push(i);
            }

            for i in 0..1000 {
                let x = imp.remove(0);
                assert_eq!(x, i);
            }
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn pop() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            for i in 0..1000 {
                imp.push(i);
            }

            for i in 0..1000 {
                let x = imp.pop().expect("issome");
                let value = 1000 - 1 - i;
                assert_eq!(value, x);
            }
            assert!(imp.pop().is_none());
        }

        test_all_pinned_types!(test);
    }

    // cons-list
    #[derive(Debug)]
    enum List<'a, T> {
        Cons(T, &'a List<'a, T>),
        Nil,
    }
    impl<'a, T> List<'a, T> {
        fn cons(&self) -> Option<&'a List<'a, T>> {
            match self {
                List::Nil => None,
                List::Cons(_, x) => Some(*x),
            }
        }
    }
    impl<'a, T: PartialEq> PartialEq for List<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Cons(l0, l1), Self::Cons(r0, r1)) => {
                    l0 == r0
                        && std::ptr::eq(l1 as *const &'a List<'a, T>, r1 as *const &'a List<'a, T>)
                }
                _ => core::mem::discriminant(self) == core::mem::discriminant(other),
            }
        }
    }
    type MyList<'a> = List<'a, usize>;
    #[test]
    fn make_cons_list() {
        fn test<'a, P>(pinned_vec: P)
        where
            P: PinnedVec<MyList<'a>> + 'a,
        {
            fn lists_maker<'a, P>(pinned_vec: P) -> ImpVec<MyList<'a>, P>
            where
                P: PinnedVec<MyList<'a>> + 'a,
            {
                let mut lists: ImpVec<MyList<'a>, P> = pinned_vec.into();
                let r0 = unsafe { lists.push_get_ref(List::Nil) };
                let r1 = unsafe { lists.push_get_ref(List::Cons(1, r0)) };
                let r2 = unsafe { lists.push_get_ref(List::Cons(2, r1)) };
                lists.push(List::Cons(3, r2));
                lists
            }

            // data
            let lists = lists_maker(pinned_vec);
            assert!(matches!(lists[0], List::Nil));
            assert!(matches!(lists[1], List::Cons(1, _)));
            assert!(matches!(lists[2], List::Cons(2, _)));
            assert!(matches!(lists[3], List::Cons(3, _)));

            // references
            assert_eq!(lists[0].cons(), None);
            assert_eq!(lists[1].cons(), Some(&lists[0]));
            assert_eq!(lists[2].cons(), Some(&lists[1]));
            assert_eq!(lists[3].cons(), Some(&lists[2]));

            // ptr-eq
            let cons_ptrs: Vec<_> = lists
                .iter()
                .map(|x| x.cons().map(|x| x as *const MyList<'a>))
                .collect();
            assert!(std::ptr::eq(
                cons_ptrs[1].expect("-"),
                &lists[0] as *const MyList<'a>
            ));
            assert!(std::ptr::eq(
                cons_ptrs[2].expect("-"),
                &lists[1] as *const MyList<'a>
            ));
            assert!(std::ptr::eq(
                cons_ptrs[3].expect("-"),
                &lists[2] as *const MyList<'a>
            ));
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn make_cons_list_as_pinned() {
        fn test<'a, G>(pinned_vec: SplitVec<MyList<'a>, G>)
        where
            G: Growth + 'a,
        {
            fn lists_maker<'a, G>(pinned_vec: SplitVec<MyList<'a>, G>) -> SplitVec<MyList<'a>, G>
            where
                G: Growth + 'a,
            {
                let mut lists: ImpVec<_, _> = pinned_vec.into();
                let r0 = unsafe { lists.push_get_ref(List::Nil) };
                let r1 = unsafe { lists.push_get_ref(List::Cons(1, r0)) };
                let r2 = unsafe { lists.push_get_ref(List::Cons(2, r1)) };
                lists.push(List::Cons(3, r2));
                lists.into()
            }

            // data
            let lists = lists_maker(pinned_vec);
            assert!(matches!(lists[0], List::Nil));
            assert!(matches!(lists[1], List::Cons(1, _)));
            assert!(matches!(lists[2], List::Cons(2, _)));
            assert!(matches!(lists[3], List::Cons(3, _)));

            // references
            assert_eq!(lists[0].cons(), None);
            assert_eq!(lists[1].cons(), Some(&lists[0]));
            assert_eq!(lists[2].cons(), Some(&lists[1]));
            assert_eq!(lists[3].cons(), Some(&lists[2]));

            // ptr-eq
            let cons_ptrs: Vec<_> = lists
                .iter()
                .map(|x| x.cons().map(|x| x as *const MyList<'a>))
                .collect();
            assert!(std::ptr::eq(
                cons_ptrs[1].expect("-"),
                &lists[0] as *const MyList<'a>
            ));
            assert!(std::ptr::eq(
                cons_ptrs[2].expect("-"),
                &lists[1] as *const MyList<'a>
            ));
            assert!(std::ptr::eq(
                cons_ptrs[3].expect("-"),
                &lists[2] as *const MyList<'a>
            ));
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn make_cons_list_as_pinned_long() {
        fn test<'a, G>(pinned_vec: SplitVec<MyList<'a>, G>)
        where
            G: Growth + 'a,
        {
            fn lists_maker<'a, G>(pinned_vec: SplitVec<MyList<'a>, G>) -> SplitVec<MyList<'a>, G>
            where
                G: Growth + 'a,
            {
                let mut lists: ImpVec<_, _> = pinned_vec.into();
                let mut last = unsafe { lists.push_get_ref(List::Nil) };
                for i in 1..10000 {
                    last = unsafe { lists.push_get_ref(List::Cons(i, last)) };
                }
                lists.into()
            }

            let lists = lists_maker(pinned_vec);
            assert_eq!(10000, lists.len());

            // data
            assert!(matches!(lists[0], List::Nil));
            for i in 1..10000 {
                assert!(matches!(lists[i], List::Cons(_, _)));
            }

            // references
            assert_eq!(lists[0].cons(), None);
            for i in 1..10000 {
                assert_eq!(lists[i].cons(), Some(&lists[i - 1]));
            }

            // ptr-eq
            let cons_ptrs: Vec<_> = lists
                .iter()
                .map(|x| x.cons().map(|x| x as *const MyList<'a>))
                .collect();
            for i in 1..10000 {
                assert!(std::ptr::eq(
                    cons_ptrs[i].expect("-"),
                    &lists[i - 1] as *const MyList<'a>
                ));
            }
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn into_split_vec() {
        fn test<G: Growth>(mut pinned_vec: SplitVec<usize, G>) {
            for i in 0..400 {
                pinned_vec.push(i * 2);
            }

            let mut imp: ImpVec<_, _> = pinned_vec.into();
            for i in 400..1000 {
                imp.push(i * 2);
            }

            let expected: Vec<_> = (0..1000).map(|i| i * 2).collect();

            assert_eq!(expected, imp);

            let pinned_back: SplitVec<usize, G> = imp.into();
            assert_eq!(expected, pinned_back);
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn collect() {
        let vec = ImpVec::<u32>::from_iter([0, 1, 2, 3, 4, 5].into_iter());
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec: ImpVec<_> = (0..6).collect();
        assert_eq!(&vec, &[0, 1, 2, 3, 4, 5]);

        let vec: ImpVec<_> = (0..6).filter(|x| x % 2 == 0).collect();
        assert_eq!(&vec, &[0, 2, 4]);
    }
}
