use orx_imp_vec::prelude::*;

#[test]
fn new_default() {
    let first = ImpVec::<char>::new();
    assert!(first.is_empty());

    let second = ImpVec::<char>::default();
    assert!(second.is_empty());

    assert_eq!(first, second);
}

#[test]
fn into_inner() {
    let impvec = ImpVec::new();
    impvec.imp_push(42);
    impvec.imp_extend_from_slice(&[7]);

    let pinned = impvec.into_inner();
    assert_eq!(&[42, 7], &pinned);
}

#[test]
fn imp_push() {
    let impvec = ImpVec::new();
    impvec.imp_push(42);

    let ref_to_first = &impvec[0];
    assert_eq!(ref_to_first, &42);

    for i in 1..56424 {
        impvec.imp_push(i);
    }

    assert_eq!(ref_to_first, &42);
    for i in 1..56424 {
        assert_eq!(i, impvec[i]);
    }
}

#[test]
fn imp_extend_from_slice() {
    let impvec = ImpVec::new();
    impvec.imp_push(42);

    let ref_to_first = &impvec[0];
    assert_eq!(ref_to_first, &42);

    for i in 1..56424 {
        impvec.imp_extend_from_slice(&[i]);
    }

    assert_eq!(ref_to_first, &42);
    for i in 1..56424 {
        assert_eq!(i, impvec[i]);
    }
}

#[test]
fn clone() {
    let impvec = ImpVec::new();
    impvec.imp_extend_from_slice(&[1, 4, 2, 1, 7]);

    let clone = impvec.clone();
    assert_eq!(&[1, 4, 2, 1, 7], &clone.into_inner());
}
