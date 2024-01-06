use crate::ImpVec;
use orx_pinned_vec::PinnedVec;
use std::ops::{Deref, DerefMut, Index, IndexMut};

impl<T, P> Index<usize> for ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.deref().get(index).expect("out-of-bounds")
    }
}
impl<T, P> IndexMut<usize> for ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.deref_mut().get_mut(index).expect("out-of-bounds")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_pinned_types;

    #[test]
    fn index() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();
            for i in 0..1000 {
                imp.push(42 + i);
            }

            for (i, val) in imp.iter().enumerate() {
                assert_eq!(42 + i, *val);
                assert_eq!(&imp[i], val);
            }
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn index_mut() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();
            for i in 0..1000 {
                imp.push(i);
            }

            for i in 0..imp.len() {
                imp[i] += 1000;
            }

            for (i, val) in imp.iter().enumerate() {
                assert_eq!(1000 + i, *val);
                assert_eq!(&imp[i], val);
            }
        }

        test_all_pinned_types!(test);
    }
}
