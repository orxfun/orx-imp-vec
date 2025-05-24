use crate::ImpVec;
use orx_concurrent_iter::IntoConcurrentIter;
use orx_fixed_vec::PinnedVec;

impl<T, P> IntoConcurrentIter for ImpVec<T, P>
where
    P: PinnedVec<T> + IntoConcurrentIter<Item = T>,
    T: Send + Sync,
{
    type Item = T;

    type IntoIter = <P as IntoConcurrentIter>::IntoIter;

    fn into_con_iter(self) -> Self::IntoIter {
        self.into_inner().into_con_iter()
    }
}
