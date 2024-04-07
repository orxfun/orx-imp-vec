use crate::imp_vec::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P: PinnedVec<T>> From<P> for ImpVec<T, P> {
    fn from(pinned_vec: P) -> Self {
        Self {
            pinned_vec: pinned_vec.into(),
            phantom: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orx_split_vec::prelude::*;

    #[test]
    fn from() {
        let mut splitvec = SplitVec::new();
        splitvec.extend_from_slice(&['a', 'b', 'c']);

        let impvec = ImpVec::from(splitvec);
        assert_eq!(*impvec, &['a', 'b', 'c']);
    }
}
