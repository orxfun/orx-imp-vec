use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P: PinnedVec<T>> FromIterator<T> for ImpVec<T, P>
where
    P: FromIterator<T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let pinned_vec: P = iter.into_iter().collect();
        pinned_vec.into()
    }
}
