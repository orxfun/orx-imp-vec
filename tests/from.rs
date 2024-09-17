use orx_imp_vec::*;

#[test]
fn from() {
    let mut vec = SplitVec::new();
    vec.extend_from_slice(&['a', 'b', 'c']);

    let imp_vec = ImpVec::from(vec);
    assert_eq!(*imp_vec, &['a', 'b', 'c']);
}
