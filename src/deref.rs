use crate::ImpVec;
use orx_split_vec::SplitVec;
use std::ops::{Deref, DerefMut};

impl<T> Deref for ImpVec<T> {
    type Target = SplitVec<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_mut_ptr() }
    }
}

impl<T> DerefMut for ImpVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
