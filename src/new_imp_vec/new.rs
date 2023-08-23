use crate::ImpVec;
use orx_split_vec::{
    CustomGrowth, DoublingGrowth, ExponentialGrowth, FixedCapacity, Fragment, LinearGrowth,
    SplitVec, SplitVecGrowth,
};
use std::rc::Rc;

pub(crate) type GetCapacityOfNewFragment<T> = dyn Fn(&[Fragment<T>]) -> usize;

impl<T, G> ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Creates an empty imp-vector with the given `growth` strategy.
    pub fn with_growth(growth: G) -> Self {
        SplitVec::with_growth(growth).into()
    }
}

impl<T> ImpVec<T, LinearGrowth> {
    /// Creates an imp-vector with linear growth and given `constant_fragment_capacity`.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the underlying `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `constant_fragment_capacity` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// // ImpVec<usize, LinearGrowth>
    /// let vec = ImpVec::with_linear_growth(16);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(16), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // push 160 elements
    /// for i in 0..10 * 16 {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(10, vec.fragments().len());
    /// for fragment in vec.fragments() {
    ///     assert_eq!(16, fragment.len());
    ///     assert_eq!(16, fragment.capacity());
    /// }
    ///
    /// // push the 161-st element
    /// vec.push(42);
    /// assert_eq!(11, vec.fragments().len());
    /// assert_eq!(Some(16), vec.fragments().last().map(|f| f.capacity()));
    /// assert_eq!(Some(1), vec.fragments().last().map(|f| f.len()));
    /// ```
    pub fn with_linear_growth(constant_fragment_capacity: usize) -> Self {
        assert!(constant_fragment_capacity > 0);
        SplitVec::with_linear_growth(constant_fragment_capacity).into()
    }
}

impl<T> ImpVec<T, DoublingGrowth> {
    /// Creates an imp-vector with doubling growth
    /// which creates a fragment with double the capacity
    /// of the prior fragment every time the split vector needs to expand.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the underlying `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `first_fragment_capacity` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// // ImpVec<usize, DoublingGrowth>
    /// let vec = ImpVec::with_doubling_growth(2);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![2, 4, 8, 16, 32];
    /// let num_items: usize = expected_fragment_capacities.iter().sum();
    /// for i in 0..num_items {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments()
    ///     .iter()
    ///     .map(|f| f.capacity())
    ///     .collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments().iter().map(|f| f.len()).collect::<Vec<_>>()
    /// );
    ///
    /// // create the 6-th fragment doubling the capacity
    /// vec.push(42);
    /// assert_eq!(
    ///     vec.fragments().len(),
    ///     expected_fragment_capacities.len() + 1
    /// );
    ///
    /// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some(32 * 2));
    /// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
    /// ```
    pub fn with_doubling_growth(first_fragment_capacity: usize) -> Self {
        assert!(first_fragment_capacity > 0);
        SplitVec::with_doubling_growth(first_fragment_capacity).into()
    }
}

impl<T> ImpVec<T, ExponentialGrowth> {
    /// Creates an imp-vector which allows new fragments grow exponentially.
    ///
    /// The capacity of the n-th fragment is computed as
    /// `cap0 * pow(growth_coefficient, n)`
    /// where `cap0` is the capacity of the first fragment.
    ///
    /// Note that `DoublingGrowth` is a special case of `ExponentialGrowth`
    /// with `growth_coefficient` equal to 2,
    /// while providing a faster access by index.
    ///
    /// On the other hand, exponential growth allows for fitting growth strategies
    /// for fitting situations which could be a better choice when memory allocation
    /// is more important than index access complexity.
    ///
    /// As you may see in the example below, it is especially useful in providing
    /// exponential growth rates slower than the doubling.
    ///
    /// Assuming it is the common case compared to empty vector scenarios,
    /// it immediately allocates the first fragment to keep the `SplitVec` struct smaller.
    ///
    /// # Panics
    /// Panics if `first_fragment_capacity` is zero,
    /// or if `growth_coefficient` is less than 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// // SplitVec<usize, ExponentialGrowth>
    /// let mut vec = ImpVec::with_exponential_growth(2, 1.5);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(2), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // fill the first 5 fragments
    /// let expected_fragment_capacities = vec![2, 3, 4, 6, 9, 13];
    /// let num_items: usize = expected_fragment_capacities.iter().sum();
    /// for i in 0..num_items {
    ///     vec.push(i);
    /// }
    ///
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments()
    ///     .iter()
    ///     .map(|f| f.capacity())
    ///     .collect::<Vec<_>>()
    /// );
    /// assert_eq!(
    ///     expected_fragment_capacities,
    ///     vec.fragments().iter().map(|f| f.len()).collect::<Vec<_>>()
    /// );
    ///
    /// // create the 6-th fragment doubling the capacity
    /// vec.push(42);
    /// assert_eq!(
    ///     vec.fragments().len(),
    ///     expected_fragment_capacities.len() + 1
    /// );
    ///
    /// assert_eq!(vec.fragments().last().map(|f| f.capacity()), Some((13 as f32 * 1.5) as usize));
    /// assert_eq!(vec.fragments().last().map(|f| f.len()), Some(1));
    /// ```
    pub fn with_exponential_growth(
        first_fragment_capacity: usize,
        growth_coefficient: f32,
    ) -> Self {
        assert!(first_fragment_capacity > 0);
        assert!(growth_coefficient >= 1.0);
        SplitVec::with_exponential_growth(first_fragment_capacity, growth_coefficient).into()
    }
}

impl<T> ImpVec<T, CustomGrowth<T>> {
    /// Creates an imp vector with the custom grwoth strategy
    /// defined by the function `get_capacity_of_new_fragment`.
    ///
    /// # Examples
    /// ```
    /// use orx_split_vec::Fragment;
    /// use orx_imp_vec::ImpVec;
    /// use std::rc::Rc;
    ///
    /// // vec: SplitVec<usize, CustomGrowth<usize>>
    /// let vec =
    ///     ImpVec::with_custom_growth_function(Rc::new(|fragments: &[Fragment<_>]| {
    ///         if fragments.len() % 2 == 0 {
    ///             2
    ///         } else {
    ///             8
    ///         }
    ///     }));
    ///
    ///     for i in 0..100 {
    ///         vec.push(i);
    ///     }
    ///
    ///     vec.into_iter().zip(0..100).all(|(l, r)| *l == r);
    ///     
    ///     for (f, fragment) in vec.fragments().iter().enumerate() {
    ///         if f % 2 == 0 {
    ///             assert_eq!(2, fragment.capacity());
    ///         } else {
    ///             assert_eq!(8, fragment.capacity());
    ///         }
    ///     }
    /// ```
    pub fn with_custom_growth_function(
        get_capacity_of_new_fragment: Rc<GetCapacityOfNewFragment<T>>,
    ) -> Self {
        SplitVec::with_custom_growth_function(get_capacity_of_new_fragment).into()
    }
}

impl<T> ImpVec<T, FixedCapacity> {
    /// Creates an imp-vector with the given `fixed_capacity`.
    ///
    /// This capacity is the hard limit and the vector cannot grow beyond it.
    /// Attempts to exceed this limit will lead to the code to panic.
    ///
    /// The benefit of this strategy, on the other hand,
    /// is its faster access by index operations;
    /// which must be inlined and have a comparable performance
    /// with regular slice access by index.
    ///
    /// Further, the pinned-memory-location of already
    /// pushed elements feature is maintained.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// // SplitVec<usize, FixedCapacity>
    /// let vec = ImpVec::with_fixed_capacity(4);
    ///
    /// assert_eq!(1, vec.fragments().len());
    /// assert_eq!(Some(4), vec.fragments().first().map(|f| f.capacity()));
    /// assert_eq!(Some(0), vec.fragments().first().map(|f| f.len()));
    ///
    /// // push 4 elements to fill the vector completely
    /// for i in 0..4 {
    ///     vec.push(i);
    /// }
    /// assert_eq!(1, vec.fragments().len());
    ///
    /// // the next push exceeding the fixed_capacity will panic.
    /// // vec.push(4);
    /// ```
    pub fn with_fixed_capacity(fixed_capacity: usize) -> Self {
        SplitVec::with_fixed_capacity(fixed_capacity).into()
    }
}
