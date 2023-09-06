#[macro_export]
#[cfg(test)]
macro_rules! test_all_pinned_types {
    ($fun:tt) => {
        #[derive(Clone, Debug)]
        pub struct DoubleEveryFourFragments;
        impl Growth for DoubleEveryFourFragments {
            fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
                let do_double = fragments.len() % 4 == 0;
                let last_capacity = fragments.last().map(|f| f.capacity()).unwrap_or(4);
                if do_double {
                    last_capacity * 2
                } else {
                    last_capacity
                }
            }
        }
        $fun(SplitVec::with_growth(DoubleEveryFourFragments));
        $fun(SplitVec::with_linear_growth(2));
        $fun(SplitVec::with_doubling_growth(2));
        $fun(SplitVec::with_exponential_growth(4, 1.5));
        $fun(SplitVec::with_exponential_growth(4, 2.5));
        $fun(FixedVec::new(1000));
    };
}

#[macro_export]
#[cfg(test)]
macro_rules! test_all_growth_types {
    ($fun:tt) => {
        #[derive(Clone, Debug)]
        pub struct DoubleEveryFourFragments;
        impl Growth for DoubleEveryFourFragments {
            fn new_fragment_capacity<T>(&self, fragments: &[Fragment<T>]) -> usize {
                let do_double = fragments.len() % 4 == 0;
                let last_capacity = fragments.last().map(|f| f.capacity()).unwrap_or(4);
                if do_double {
                    last_capacity * 2
                } else {
                    last_capacity
                }
            }
        }
        $fun(SplitVec::with_growth(DoubleEveryFourFragments));
        $fun(SplitVec::with_linear_growth(2));
        $fun(SplitVec::with_doubling_growth(2));
        $fun(SplitVec::with_exponential_growth(4, 1.5));
        $fun(SplitVec::with_exponential_growth(4, 2.5));
    };
}
