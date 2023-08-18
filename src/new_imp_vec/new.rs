use crate::ImpVec;
use orx_split_vec::{FragmentGrowth, SplitVec};

impl<T> ImpVec<T> {
    /// Creates an empty imp-vector with the given `growth` strategy.
    pub fn with_growth(growth: FragmentGrowth) -> Self {
        SplitVec::with_growth(growth).into()
    }
}
