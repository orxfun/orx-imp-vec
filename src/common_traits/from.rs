use crate::imp_vec::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P: PinnedVec<T>> From<P> for ImpVec<T, P> {
    fn from(pinned_vec: P) -> Self {
        Self {
            pinned_vec: pinned_vec.into(),
            phantom: Default::default(),
        }
    }
}
