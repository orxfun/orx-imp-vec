use crate::prelude::*;
use orx_fixed_vec::FixedVec;

#[test]
fn eq() {
    let mut split = SplitVec::new();
    split.extend_from_slice(&['a', 'b', 'c']);
    let vec1 = ImpVec::from(split);

    let mut fixed = FixedVec::new(10);
    fixed.extend_from_slice(&['a', 'b', 'c']);
    let vec2 = ImpVec::from(fixed);

    assert_eq!(&vec1, &vec2);
    assert_eq!(vec1, vec2);
}
