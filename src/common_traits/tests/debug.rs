use crate::prelude::*;

#[test]
fn debug() {
    let vec = ImpVec::new();
    vec.imp_extend_from_slice(&['a', 'b', 'c']);

    let expected_debug_str = format!("ImpVec {{ pinned_vec: {:?} }}", &vec.pinned_vec);
    assert_eq!(expected_debug_str, format!("{:?}", &vec));
}
