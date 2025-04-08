use orx_fixed_vec::FixedVec;
use orx_imp_vec::*;

#[test]
fn eq() {
    #[cfg(not(miri))]
    let range = || 13..542;
    #[cfg(miri)]
    let range = || 13..67;

    let vec1: ImpVec<_> = (range()).map(|x| x.to_string()).collect();

    let other: ImpVec<_, FixedVec<_>> = (range()).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: SplitVec<_, Doubling> = (range()).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: SplitVec<_, Recursive> = (range()).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: FixedVec<_> = (range()).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: Vec<_> = (range()).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, &other);
    assert_eq!(&other, &vec1);

    let other: Vec<_> = (range()).map(|x| x.to_string()).collect();
    assert_eq!(&vec1, other.as_slice());
    assert_eq!(other.as_slice(), &vec1);
}
