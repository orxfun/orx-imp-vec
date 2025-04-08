use orx_imp_vec::*;

#[test]
fn from_iter() {
    fn validate<P: PinnedVec<String>>(imp: ImpVec<String, P>) {
        for (i, x) in imp.iter().enumerate() {
            assert_eq!(*x, i.to_string());
        }
    }

    #[cfg(not(miri))]
    let n = 845;
    #[cfg(miri)]
    let n = 57;

    let imp: ImpVec<_> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);

    let imp: ImpVec<_, SplitVec<_, Doubling>> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);

    let imp: ImpVec<_, SplitVec<_, Recursive>> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);

    let imp: ImpVec<_, FixedVec<_>> = (0..n).map(|x| x.to_string()).collect();
    validate(imp);
}
