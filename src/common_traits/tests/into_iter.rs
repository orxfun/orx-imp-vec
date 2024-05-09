use crate::prelude::*;
use orx_fixed_vec::FixedVec;

#[test]
fn into_iter() {
    let mut splitvec = SplitVec::new();
    splitvec.extend_from_slice(&['a', 'b', 'c'].map(|x| x.to_string()));
    let impvec = ImpVec::from(splitvec);

    let into_iter = impvec.into_iter();
    let vec: Vec<String> = into_iter.collect();

    let fixed: FixedVec<String> = vec.into();
    let impvec = ImpVec::from(fixed);
    let mut into_iter = impvec.into_iter();

    assert_eq!(into_iter.next(), Some(String::from("a")));
    assert_eq!(into_iter.next(), Some(String::from("b")));
    assert_eq!(into_iter.next(), Some(String::from("c")));
    assert_eq!(into_iter.next(), None);
}
