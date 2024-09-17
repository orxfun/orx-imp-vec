use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P: PinnedVec<T>> IntoIterator for ImpVec<T, P> {
    type Item = T;
    type IntoIter = P::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}
