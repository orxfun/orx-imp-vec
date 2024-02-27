use crate::imp_vec::ImpVec;
use orx_pinned_vec::PinnedVec;
use std::ops::{Deref, DerefMut};

impl<T, P: PinnedVec<T>> Deref for ImpVec<T, P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        &self.pinned_vec
    }
}

impl<T, P: PinnedVec<T>> DerefMut for ImpVec<T, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pinned_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref() {
        let vec = ImpVec::new();
        vec.imp_extend_from_slice(&['a', 'b', 'c']);

        let pinned_deref = vec.deref();
        assert_eq!(pinned_deref, &vec.pinned_vec);
    }

    #[test]
    fn deref_mut() {
        let mut vec = ImpVec::new();
        vec.imp_extend_from_slice(&['a', 'b', 'c']);

        let pinned_deref = vec.deref_mut();
        pinned_deref.push('d');

        assert_eq!('d', vec[3]);
    }
}
