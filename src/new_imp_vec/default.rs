use crate::ImpVec;
use orx_split_vec::SplitVec;
use std::cell::RefCell;

impl<T> Default for ImpVec<T> {
    /// Creates an empty imp-vector with the default `FragmentGrowth` strategy.
    fn default() -> Self {
        Self {
            split_vec: RefCell::new(SplitVec::default()),
        }
    }
}
