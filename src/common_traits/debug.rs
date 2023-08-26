use std::fmt::{Debug, Formatter, Result};

use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

impl<T, P> Debug for ImpVec<T, P>
where
    P: PinnedVec<T>,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.cell.borrow().debug(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;
    use std::fmt::Debug;

    #[test]
    fn debug() {
        fn test<P: PinnedVec<usize> + Debug>(mut pinned_vec: P) {
            for i in 0..17 {
                pinned_vec.push(i);
            }
            let expected_debug = format!("{:?}", pinned_vec);

            let imp: ImpVec<_, _> = pinned_vec.into();
            assert_eq!(expected_debug, format!("{:?}", imp));
        }

        test_all_growth_types!(test);
    }
}
