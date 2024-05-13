use crate::prelude::*;

#[test]
fn debug() {
    let vec = ImpVec::new();
    for i in 0..10 {
        vec.imp_push(i.to_string());
    }

    let imp_vec_str = format!("{:?}", &vec);

    let pinned_vec = vec.into_inner();
    let pinned_vec_str = format!("{:?}", &pinned_vec);
    let expected_str = format!("ImpVec {{ pinned_vec: {} }}", pinned_vec_str);

    assert_eq!(imp_vec_str, expected_str);
}
