use crate::prelude::*;
use orx_fixed_vec::FixedVec;

#[test]
fn eq() {
    let vec1: ImpVec<_> = (13..542).map(|x| x.to_string()).collect();

    let other: ImpVec<_, FixedVec<_>> = (13..542).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: SplitVec<_, Doubling> = (13..542).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: SplitVec<_, Recursive> = (13..542).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: FixedVec<_> = (13..542).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: Vec<_> = (13..542).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: Vec<_> = (13..542).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, other.as_slice());
    assert_eq!(other.as_slice(), &vec1);
}
