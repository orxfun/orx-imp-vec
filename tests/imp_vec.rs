use orx_imp_vec::*;

#[test]
fn new_default() {
    // TODO: implement safe ref methods

    let first = ImpVec::<char>::new();
    // assert!(first.is_empty());

    // let second = ImpVec::<char>::default();
    // assert!(second.is_empty());

    // assert_eq!(first, second);
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

    let elem_first = imp_vec.get(0).unwrap();
    assert_eq!(elem_first.copied(), 42);

    #[cfg(not(miri))]
    let until = 56424;
    #[cfg(miri)]
    let until = 34;

    for i in 1..until {
        imp_vec.imp_push(i);
    }

    assert_eq!(elem_first.copied(), 42);
    for i in 1..until {
        assert_eq!(imp_vec.get(i).unwrap().copied(), i);
    }
}

#[test]
fn imp_extend_from_slice() {
    let imp_vec = ImpVec::new();
    imp_vec.imp_push(42);

    let elem_first = imp_vec.get(0).unwrap();
    assert_eq!(elem_first.copied(), 42);

    #[cfg(not(miri))]
    let until = 56424;
    #[cfg(miri)]
    let until = 34;

    for i in 1..until {
        imp_vec.imp_extend_from_slice(&[i]);
    }

    assert_eq!(elem_first.copied(), 42);
    for i in 1..until {
        assert_eq!(imp_vec.get(i).unwrap().copied(), i);
    }
}

#[test]
fn clone() {
    let imp_vec = ImpVec::new();
    imp_vec.imp_extend_from_slice(&[1, 4, 2, 1, 7]);

    let clone = imp_vec.clone();
    assert_eq!(&[1, 4, 2, 1, 7], &clone.into_inner());
}
