#[macro_export]
#[cfg(test)]
macro_rules! test_all_growth_types {
    ($fun:tt) => {
        let custom_fun = std::rc::Rc::new(
            |fragments: &[Fragment<_>]| {
                if fragments.len() % 2 == 0 {
                    2
                } else {
                    8
                }
            },
        );
        $fun(SplitVec::with_custom_growth_function(custom_fun));
        $fun(SplitVec::with_linear_growth(2));
        $fun(SplitVec::with_doubling_growth(2));
        $fun(SplitVec::with_exponential_growth(4, 1.5));
        $fun(SplitVec::with_exponential_growth(4, 2.5));
        $fun(FixedVec::new(1000));
    };
}
