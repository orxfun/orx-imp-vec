use crate::imp_vec::ImpVec;
use core::fmt::Debug;
use orx_pinned_vec::PinnedVec;

impl<T: Debug, P: PinnedVec<T> + Debug> Debug for ImpVec<T, P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.iter();
        if let Some(x) = iter.next() {
            write!(f, "{:?}", x)?;
            for x in iter {
                write!(f, ", {:?}", x)?;
            }
        }
        write!(f, "]")
    }
}
