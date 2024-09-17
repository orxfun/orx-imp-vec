pub use orx_imp_vec::*;

#[test]
fn index() {
    let vec = ImpVec::new();
    vec.imp_push(0);
    vec.imp_extend_from_slice([1, 2, 3, 4].as_slice());

    assert_eq!(&vec, [0, 1, 2, 3, 4].as_slice());

    for i in 0..vec.len() {
        assert_eq!(vec[i], i);
    }
}

#[test]
fn index_mut() {
    let mut vec = ImpVec::new();
    vec.imp_push(0);
    vec.imp_extend_from_slice([1, 2, 3, 4].as_slice());

    for i in 0..vec.len() {
        vec[i] *= 2;
    }

    assert_eq!(&vec, [0, 2, 4, 6, 8].as_slice());
}
