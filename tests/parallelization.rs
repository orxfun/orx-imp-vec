use orx_imp_vec::*;
use orx_parallel::*;
use std::format;
use test_case::test_matrix;

#[cfg(miri)]
const N: usize = 125;
#[cfg(not(miri))]
const N: usize = 4735;

#[test_matrix(
    [0, 1, N],
    [0, 1, 4]
)]
fn par_map_filter_collect(n: usize, nt: usize) {
    let vec = ImpVec::new();
    for i in 0..n {
        vec.imp_push((i + 10).to_string());
    }

    let expected: Vec<_> = vec
        .iter()
        .map(|x| format!("{}!", x))
        .filter(|x| !x.starts_with('1'))
        .collect();

    let result_ref: Vec<_> = vec
        .par()
        .num_threads(nt)
        .map(|x| format!("{}!", x))
        .filter(|x| !x.starts_with('1'))
        .collect();

    let result_owned: Vec<_> = vec
        .into_par()
        .num_threads(nt)
        .map(|x| format!("{}!", x))
        .filter(|x| !x.starts_with('1'))
        .collect();

    assert_eq!(expected, result_ref);
    assert_eq!(expected, result_owned);
}

#[test]
fn pass_as_parallelizable_collection() {
    let n = 64;
    let vec = ImpVec::new();
    for i in 0..n {
        vec.imp_push((i + 10).to_string());
    }

    fn take_ref<C: ParallelizableCollection<Item = String>>(vec: &C) -> Vec<String> {
        vec.par()
            .map(|x| format!("{}!", x))
            .filter(|x| !x.starts_with('1'))
            .collect()
    }

    let expected: Vec<_> = vec
        .iter()
        .map(|x| format!("{}!", x))
        .filter(|x| !x.starts_with('1'))
        .collect();

    let result_ref = take_ref(&vec);

    assert_eq!(expected, result_ref);
}
