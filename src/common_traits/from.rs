use crate::ImpVec;
use orx_pinned_vec::PinnedVec;
use std::cell::RefCell;

impl<T, P> From<P> for ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    fn from(value: P) -> Self {
        Self::new(RefCell::new(value))
    }
}
