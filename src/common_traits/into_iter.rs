use crate::prelude::*;

impl<T, P: PinnedVec<T>> IntoIterator for ImpVec<T, P> {
    type Item = T;
    type IntoIter = P::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inner().into_iter()
    }
}
