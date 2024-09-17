use crate::imp_vec::ImpVec;
use core::ops::{Index, IndexMut};
use orx_pinned_vec::PinnedVec;

const OOB: &str = "out-of-bounds";

impl<T, P: PinnedVec<T>> Index<usize> for ImpVec<T, P> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect(OOB)
    }
}

impl<T, P: PinnedVec<T>> IndexMut<usize> for ImpVec<T, P> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect(OOB)
    }
}
