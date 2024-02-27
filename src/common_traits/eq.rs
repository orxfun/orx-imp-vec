use crate::imp_vec::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T: PartialEq, P1: PinnedVec<T>, P2: PinnedVec<T>> PartialEq<ImpVec<T, P2>> for ImpVec<T, P1> {
    fn eq(&self, other: &ImpVec<T, P2>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
