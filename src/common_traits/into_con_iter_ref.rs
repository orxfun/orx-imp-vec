use crate::ImpVec;
use orx_concurrent_iter::IntoConcurrentIter;
use orx_fixed_vec::PinnedVec;

impl<'a, T, P> IntoConcurrentIter for &'a ImpVec<T, P>
where
    P: PinnedVec<T>,
    &'a P: IntoConcurrentIter<Item = &'a T>,
    T: Send + Sync,
{
    type Item = &'a T;

    type IntoIter = <&'a P as IntoConcurrentIter>::IntoIter;

    fn into_con_iter(self) -> Self::IntoIter {
        self.pinned().into_con_iter()
    }
}
