#[macro_export]
#[cfg(test)]
macro_rules! test_all_growth_types {
    ($fun:tt) => {
        let custom_fun = std::rc::Rc::new(
            |fragments: &[orx_split_vec::Fragment<usize>]| {
                if fragments.len() % 2 == 0 {
                    2
                } else {
                    8
                }
            },
        );
        $fun::<orx_split_vec::CustomGrowth<usize>>(ImpVec::with_custom_growth_function(custom_fun));
        $fun::<orx_split_vec::LinearGrowth>(ImpVec::with_linear_growth(2));
        $fun::<orx_split_vec::DoublingGrowth>(ImpVec::with_doubling_growth(2));
        $fun::<orx_split_vec::ExponentialGrowth>(ImpVec::with_exponential_growth(4, 1.5));
    };
}
