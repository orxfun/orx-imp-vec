use orx_imp_vec::*;

#[test]
fn debug() {
    let vec = ImpVec::new();
    for i in 0..10 {
        vec.imp_push(i.to_string());
    }

    let imp_vec_str = format!("{:?}", &vec);
    let expected_str =
        format!("[\"0\", \"1\", \"2\", \"3\", \"4\", \"5\", \"6\", \"7\", \"8\", \"9\"]");

    assert_eq!(imp_vec_str, expected_str);
}
