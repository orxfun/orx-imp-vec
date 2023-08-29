use crate::ImpVec;
use orx_fixed_vec::FixedVec;
use orx_split_vec::{SplitVec, SplitVecGrowth};

impl<T> From<ImpVec<T, FixedVec<T>>> for FixedVec<T> {
    fn from(value: ImpVec<T, FixedVec<T>>) -> Self {
        value.cell.into_inner()
    }
}

impl<T, G> From<ImpVec<T, SplitVec<T, G>>> for SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    fn from(value: ImpVec<T, SplitVec<T, G>>) -> Self {
        value.cell.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn into_split_vec() {
        fn test<G: SplitVecGrowth<usize>>(mut pinned_vec: SplitVec<usize, G>) {
            for i in 0..400 {
                pinned_vec.push(i * 2);
            }

            let imp: ImpVec<_, _> = pinned_vec.into();
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
    fn into_fixed_vec() {
        let mut pinned_vec = FixedVec::new(1000);

        for i in 0..400 {
            pinned_vec.push(i * 2);
        }

        let imp: ImpVec<_, _> = pinned_vec.into();
        for i in 400..1000 {
            imp.push(i * 2);
        }

        let expected: Vec<_> = (0..1000).map(|i| i * 2).collect();

        assert_eq!(expected, imp);

        let pinned_back: FixedVec<usize> = imp.into();
        assert_eq!(pinned_back, expected);
    }
}
