use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P> Clone for ImpVec<T, P>
where
    P: PinnedVec<T> + Clone,
{
    fn clone(&self) -> Self {
        let pinned_vec = unsafe { &mut *self.pinned_vec.get() }.clone();
        Self {
            pinned_vec: pinned_vec.into(),
            phantom: self.phantom,
        }
    }
}
