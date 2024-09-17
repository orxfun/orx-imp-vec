use orx_imp_vec::*;
use std::ops::{Deref, DerefMut};

#[test]
fn deref() {
    let vec = ImpVec::new();
    vec.imp_extend_from_slice(&['a', 'b', 'c']);

    let pinned_deref = vec.deref();
    assert_eq!(pinned_deref, &['a', 'b', 'c']);
}

#[test]
fn deref_mut() {
    let mut vec = ImpVec::new();
    vec.imp_extend_from_slice(&['a', 'b', 'c']);

    let pinned_deref = vec.deref_mut();
    pinned_deref.push('d');

    assert_eq!('d', vec[3]);
}
