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
