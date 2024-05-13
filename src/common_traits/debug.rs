use crate::imp_vec::ImpVec;
use orx_pinned_vec::PinnedVec;
use std::fmt::Debug;

impl<T, P: PinnedVec<T> + Debug> Debug for ImpVec<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pinned_vec = unsafe { &*self.pinned_vec.get() };
        f.debug_struct("ImpVec")
            .field("pinned_vec", &pinned_vec)
            .finish()
    }
}
