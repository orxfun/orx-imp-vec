use orx_imp_vec::*;

#[test]
fn from_iter() {
    fn validate<P: PinnedVec<String>>(imp: ImpVec<String, P>) {
        for (i, x) in imp.iter().enumerate() {
            assert_eq!(*x, i.to_string());
        }
    }

    let n = 845;

    let imp: ImpVec<_> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);

    let imp: ImpVec<_, SplitVec<_, Doubling>> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);

    let imp: ImpVec<_, SplitVec<_, Recursive>> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);

    let imp: ImpVec<_, FixedVec<_>> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);
}
