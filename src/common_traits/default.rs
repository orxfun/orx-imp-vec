use crate::ImpVec;

impl<T> Default for ImpVec<T> {
    /// Creates a new empty imp-vec.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let imp_vec: ImpVec<usize> = ImpVec::default();
    /// assert!(imp_vec.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}
