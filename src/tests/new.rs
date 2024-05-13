use crate::prelude::*;

#[test]
fn new() {
    fn test<P: PinnedVec<String>>(vec: ImpVec<String, P>) {
        for i in 13..542 {
            assert_eq!(vec.get(i - 13), Some(&i.to_string()));
        }
    }

    let imp = ImpVec::default();
    for i in 13..542 {
        imp.imp_push(i.to_string());
    }
    test(imp);

    let imp = ImpVec::new();
    for i in 13..542 {
        imp.imp_push(i.to_string());
    }
    test(imp);

    let imp = ImpVec::with_doubling_growth();
    for i in 13..542 {
        imp.imp_push(i.to_string());
    }
    test(imp);

    let imp = ImpVec::with_recursive_growth();
    for i in 13..542 {
        imp.imp_push(i.to_string());
    }
    test(imp);

    let imp = ImpVec::with_linear_growth(7);
    for i in 13..542 {
        imp.imp_push(i.to_string());
    }
    test(imp);

    let imp = ImpVec::with_fixed_capacity(1000);
    for i in 13..542 {
        imp.imp_push(i.to_string());
    }
    test(imp);
}
