use crate::prelude::*;

#[test]
fn from() {
    let mut splitvec = SplitVec::new();
    splitvec.extend_from_slice(&['a', 'b', 'c']);

    let impvec = ImpVec::from(splitvec);
    assert_eq!(*impvec, &['a', 'b', 'c']);
}
