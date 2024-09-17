use crate::imp_vec::ImpVec;
use core::ops::{Deref, DerefMut};
use orx_pinned_vec::PinnedVec;

impl<T, P: PinnedVec<T>> Deref for ImpVec<T, P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        self.pinned_mut()
    }
}

impl<T, P: PinnedVec<T>> DerefMut for ImpVec<T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pinned_mut()
    }
}
