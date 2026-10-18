use crate::imp_vec::ImpVec;
use core::ops::{Deref, DerefMut};
use orx_pinned_vec::PinnedVec;

impl<T, P: PinnedVec<T>> Deref for ImpVec<T, P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        self.pinned_mut()
    }
}

// TODO: implement mut methods that are not removing elements from the vector.
