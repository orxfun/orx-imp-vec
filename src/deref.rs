use crate::ImpVec;
use orx_split_vec::{SplitVec, SplitVecGrowth};
use std::ops::{Deref, DerefMut};

impl<T, G> Deref for ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    type Target = SplitVec<T, G>;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.as_mut_ptr() }
    }
}

impl<T, G> DerefMut for ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.as_mut_ptr() }
    }
}
