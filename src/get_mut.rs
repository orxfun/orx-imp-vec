use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P> ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut T> {
        let data = self.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            pinned_vec.get_mut(index)
        }
    }
}
