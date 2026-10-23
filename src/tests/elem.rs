use crate::ImpVec;
use alloc::vec;

#[test]
fn elem_api() {
    let v = ImpVec::new();
    let other_v = ImpVec::<usize>::new();

    let mut elements = vec![];
    for i in 0..10 {
        elements.push(v.push(i));
    }

    for (i, e) in elements.iter().enumerate() {
        assert_eq!(i, *unsafe { e.get() });
        assert_eq!(i, e.copied());
        assert_eq!(i, e.cloned());
        assert_eq!(i + 1, e.map(|x| x + 1));
        assert_eq!(Some(i), e.position_in(&v));
        assert_eq!(None, e.position_in(&other_v));

        let x = e.replace(i + 1);
        assert_eq!(x, i);
        assert_eq!(i + 1, *unsafe { e.get() });

        e.set(i + 2);
        assert_eq!(i + 2, *unsafe { e.get() });

        e.update(|x| *x -= 2);
        assert_eq!(i, *unsafe { e.get() });
    }
}
