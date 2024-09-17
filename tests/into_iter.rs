use orx_fixed_vec::FixedVec;
use orx_imp_vec::*;

#[test]
fn into_iter() {
    let mut vec = SplitVec::new();
    vec.extend_from_slice(&['a', 'b', 'c'].map(|x| x.to_string()));
    let imp_vec = ImpVec::from(vec);

    let into_iter = imp_vec.into_iter();
    let vec: Vec<String> = into_iter.collect();

    let fixed: FixedVec<String> = vec.into();
    let imp_vec = ImpVec::from(fixed);
    let mut into_iter = imp_vec.into_iter();

    assert_eq!(into_iter.next(), Some(String::from("a")));
    assert_eq!(into_iter.next(), Some(String::from("b")));
    assert_eq!(into_iter.next(), Some(String::from("c")));
    assert_eq!(into_iter.next(), None);
}
