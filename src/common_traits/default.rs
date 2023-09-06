use crate::ImpVec;
use orx_pinned_vec::PinnedVec;
use std::cell::RefCell;

impl<T, P> Default for ImpVec<T, P>
where
    P: PinnedVec<T> + Default,
{
    fn default() -> Self {
        Self::new(RefCell::new(Default::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn default() {
        fn test<P: PinnedVec<char> + Default>() {
            let default_pin = P::default();
            let default_imp = ImpVec::<char, P>::default();
            let imp_from_default_pin: ImpVec<_, _> = default_pin.into();
            assert_eq!(default_imp, imp_from_default_pin);
        }

        test::<SplitVec<char, Linear>>();
        test::<SplitVec<char, Doubling>>();
        test::<SplitVec<char, Exponential>>();
    }
}
