use crate::ImpVec;
use orx_fixed_vec::FixedVec;
use orx_split_vec::{Doubling, Linear, Recursive, SplitVec};

impl<T> ImpVec<T> {
    /// Creates a new empty imp-vec.
    ///
    /// Default underlying pinned vector is a new [`SplitVec<T, Doubling>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Doubling.html).
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_imp_vec::prelude::*;
    ///
    /// let imp_vec: ImpVec<char> = ImpVec::new();
    /// assert!(imp_vec.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            pinned_vec: SplitVec::default().into(),
            phantom: Default::default(),
        }
    }
}

impl<T> ImpVec<T, SplitVec<T, Doubling>> {
    /// Creates a new ImpVec by creating and wrapping up a new [`SplitVec<T, Doubling>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Doubling.html) as the underlying storage.
    pub fn with_doubling_growth() -> Self {
        SplitVec::with_doubling_growth().into()
    }
}

impl<T> ImpVec<T, SplitVec<T, Recursive>> {
    /// Creates a new ImpVec by creating and wrapping up a new [`SplitVec<T, Recursive>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Recursive.html) as the underlying storage.
    pub fn with_recursive_growth() -> Self {
        SplitVec::with_recursive_growth().into()
    }
}

impl<T> ImpVec<T, SplitVec<T, Linear>> {
    /// Creates a new ImpVec by creating and wrapping up a new [`SplitVec<T, Linear>`](https://docs.rs/orx-split-vec/latest/orx_split_vec/struct.Linear.html) as the underlying storage.
    ///
    /// * Each fragment of the underlying split vector will have a capacity of  `2 ^ constant_fragment_capacity_exponent`.
    pub fn with_linear_growth(constant_fragment_capacity_exponent: usize) -> Self {
        SplitVec::with_linear_growth(constant_fragment_capacity_exponent).into()
    }
}

impl<T> ImpVec<T, FixedVec<T>> {
    /// Creates a new ImpVec by creating and wrapping up a new [`FixedVec<T>`]((https://docs.rs/orx-fixed-vec/latest/orx_fixed_vec/)) as the underlying storage.
    ///
    /// # Safety
    ///
    /// Note that a `FixedVec` cannot grow beyond the given `fixed_capacity`.
    /// In other words, has a hard upper bound on the number of elements it can hold, which is the `fixed_capacity`.
    ///
    /// Pushing to the vector beyond this capacity leads to "out-of-capacity" error.
    ///
    /// This maximum capacity can be accessed by the `capacity`method.
    pub fn with_fixed_capacity(fixed_capacity: usize) -> Self {
        FixedVec::new(fixed_capacity).into()
    }
}
