use orx_imp_vec::*;
use std::ops::Deref;

#[test]
fn deref() {
    let vec = ImpVec::new();
    vec.imp_extend_from_slice(&['a', 'b', 'c']);

    let pinned_deref = vec.deref();
    assert_eq!(pinned_deref, &['a', 'b', 'c']);
}
