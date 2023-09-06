use crate::ImpVec;
use orx_pinned_vec::PinnedVec;
use orx_split_vec::{Growth, SplitVec};

impl<T, P1, P2> PartialEq<ImpVec<T, P2>> for ImpVec<T, P1>
where
    P1: PinnedVec<T>,
    P2: PinnedVec<T>,
    T: PartialEq,
{
    fn eq(&self, other: &ImpVec<T, P2>) -> bool {
        if self.len() == other.len() {
            self.iter().zip(other.iter()).all(|(x, y)| x == y)
        } else {
            false
        }
    }
}
impl<T, P> Eq for ImpVec<T, P>
where
    P: PinnedVec<T>,
    T: PartialEq,
{
}

impl<T, P, S> PartialEq<S> for ImpVec<T, P>
where
    P: PinnedVec<T>,
    S: AsRef<[T]>,
    T: PartialEq,
{
    fn eq(&self, other: &S) -> bool {
        self.cell.borrow().partial_eq(other.as_ref())
    }
}

impl<T, P> PartialEq<ImpVec<T, P>> for Vec<T>
where
    P: PinnedVec<T>,
    T: PartialEq,
{
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        other.cell.borrow().partial_eq(self)
    }
}
impl<T, P, const N: usize> PartialEq<ImpVec<T, P>> for [T; N]
where
    P: PinnedVec<T>,
    T: PartialEq,
{
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        other.cell.borrow().partial_eq(self)
    }
}

impl<T, P, G> PartialEq<ImpVec<T, P>> for SplitVec<T, G>
where
    P: PinnedVec<T>,
    T: PartialEq,
    G: Growth,
{
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_pinned_types;

    #[test]
    fn eq_with_imp() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp1: ImpVec<_, _> = pinned_vec.into();
            let imp2: ImpVec<_, _> = FixedVec::new(3344).into();

            for i in 0..1000 {
                imp1.push(i);
                imp2.push(i);
            }
            assert_eq!(imp1, imp2);
            assert_eq!(imp2, imp1);

            imp2.push(1000);
            assert_ne!(imp1, imp2);
            assert_ne!(imp2, imp1);
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn eq_with_asref() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            let mut vec = vec![];

            for i in 0..1000 {
                imp.push(i);
                vec.push(i);
            }
            assert_eq!(vec, imp);
            assert_eq!(imp, vec);
            assert_eq!(imp, vec.as_slice());
            assert_eq!(imp, &vec);
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn eq_with_slice() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            for i in 0..4 {
                imp.push(i);
            }
            assert_eq!(vec![0, 1, 2, 3], imp);
            assert_eq!(imp, &[0, 1, 2, 3]);
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn eq_with_array() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            for i in 0..4 {
                imp.push(i);
            }
            assert_eq!([0, 1, 2, 3], imp);
            assert_eq!(imp, [0, 1, 2, 3]);
        }

        test_all_pinned_types!(test);
    }

    #[test]
    fn eq_with_split() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            let mut split = SplitVec::new();
            for i in 0..42 {
                imp.push(i);
                split.push(i);
            }
            assert_eq!(split, imp);
        }

        test_all_pinned_types!(test);
    }
}
