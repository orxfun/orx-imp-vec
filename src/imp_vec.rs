use orx_pinned_vec::PinnedVec;
use orx_split_vec::SplitVec;
use std::{cell::RefCell, marker::PhantomData};

/// `ImpVec` stands for 'immutable-push-vec'.
///
/// It may `orx_split_vec::SplitVec` as the underlying data structure providing
/// flexible growth strategies, or `orx_fixed_vec::FixedVec` with a preset strict capacity
/// while providing standard vector complexity in common operations.
///
/// In addition, it allows to push/extend the vector with an immutable reference.
///
/// This allows to hold on the references of already pushed elements
/// while building the collection.
pub struct ImpVec<T, P = SplitVec<T>>
where
    P: PinnedVec<T>,
{
    pub(crate) cell: RefCell<P>,
    phantom: PhantomData<T>,
}

impl<T, P> ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    pub(crate) fn new(split_vec: RefCell<P>) -> Self {
        Self {
            cell: split_vec,
            phantom: Default::default(),
        }
    }
    pub(crate) fn as_mut_ptr(&self) -> *mut P {
        self.cell.as_ptr()
    }
}

// into
impl<T, P> From<ImpVec<T, P>> for RefCell<P>
where
    P: PinnedVec<T>,
{
    fn from(value: ImpVec<T, P>) -> Self {
        value.cell
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::prelude::*;
    use crate::test_all_pinned_types;

    #[test]
    fn as_mut_ptr() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            let ptr = imp.as_mut_ptr();
            for i in 0..1000 {
                imp.push(i);
            }

            assert_eq!(ptr, imp.as_mut_ptr());
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn into_pined() {
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

        let imp: ImpVec<char> = SplitVec::with_initial_capacity(2).into();
        assert_eq!(SplitVec::with_initial_capacity(2), imp);

        let imp: ImpVec<char, FixedVec<char>> = FixedVec::new(10).into();
        assert_eq!(imp, FixedVec::new(42));

        let imp: ImpVec<char, SplitVec<char, Linear>> = SplitVec::with_linear_growth(2).into();
        assert_eq!(SplitVec::with_doubling_growth(22), imp);

        let imp: ImpVec<char, SplitVec<char, Doubling>> = SplitVec::with_doubling_growth(2).into();
        assert_eq!(SplitVec::with_linear_growth(22), imp);

        let imp: ImpVec<char, SplitVec<char, Exponential>> =
            SplitVec::with_exponential_growth(2, 1.25).into();
        assert_eq!(imp, FixedVec::new(33));
    }
}
