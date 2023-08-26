use std::ops::{Deref, DerefMut};

use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P> Deref for ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    type Target = P;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_mut_ptr() }
    }
}

impl<T, P> DerefMut for ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
