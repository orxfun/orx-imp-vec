use orx_imp_vec::*;

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
    let imp_vec = ImpVec::new();
    imp_vec.imp_push(42);
    imp_vec.imp_extend_from_slice(&[7]);

    let pinned = imp_vec.into_inner();
    assert_eq!(&[42, 7], &pinned);
}

#[test]
fn imp_push() {
    let imp_vec = ImpVec::new();
    imp_vec.imp_push(42);

    let ref_to_first = &imp_vec[0];
    assert_eq!(ref_to_first, &42);

    #[cfg(not(miri))]
    let until = 56424;
    #[cfg(miri)]
    let until = 34;

    for i in 1..until {
        imp_vec.imp_push(i);
    }

    assert_eq!(ref_to_first, &42);
    for i in 1..until {
        assert_eq!(i, imp_vec[i]);
    }
}

#[test]
fn imp_extend_from_slice() {
    let imp_vec = ImpVec::new();
    imp_vec.imp_push(42);

    let ref_to_first = &imp_vec[0];
    assert_eq!(ref_to_first, &42);

    #[cfg(not(miri))]
    let until = 56424;
    #[cfg(miri)]
    let until = 34;

    for i in 1..until {
        imp_vec.imp_extend_from_slice(&[i]);
    }

    assert_eq!(ref_to_first, &42);
    for i in 1..until {
        assert_eq!(i, imp_vec[i]);
    }
}

#[test]
fn clone() {
    let imp_vec = ImpVec::new();
    imp_vec.imp_extend_from_slice(&[1, 4, 2, 1, 7]);

    let clone = imp_vec.clone();
    assert_eq!(&[1, 4, 2, 1, 7], &clone.into_inner());
}
