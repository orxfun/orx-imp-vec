use crate::ImpVec;
use orx_fixed_vec::FixedVec;
use orx_pinned_vec::PinnedVec;
use orx_split_vec::{Growth, SplitVec};

impl<T> From<ImpVec<T, FixedVec<T>>> for FixedVec<T> {
    fn from(value: ImpVec<T, FixedVec<T>>) -> Self {
        value.into_pinned()
    }
}

impl<T, G> From<ImpVec<T, SplitVec<T, G>>> for SplitVec<T, G>
where
    G: Growth,
{
    fn from(value: ImpVec<T, SplitVec<T, G>>) -> Self {
        value.into_pinned()
    }
}

impl<T, P> ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    /// Converts the ImpVec into underlying PinnedVec.
    ///
    /// This operation corresponds to moving the pinned vec out of a ref cell; and hence, is not expensive.
    /// Likewise, the pinned vec can then be converted back into an imp vec by wrapping it up by a ref cell under the hood.
    ///
    /// Converting an ImpVec to PinnedVec is useful for the following reasons:
    ///
    /// * It reduces one level of abstraction by getting rid of the ref cell, and hence,
    /// achieving SplitVec or FixedVec (same as std vec) performance depending on the type of the pinned vec.
    /// * It means that immutable pushes are not allowed anymore.
    ///     * In a vector holding inter-element references, this means that such references will not be added or removed.
    ///     * In other words, it safely ensures that the building of the structure is completed.
    ///
    /// # Examples
    ///
    /// You may see and example back-and-forth conversions between a PinnedVec.
    ///
    /// *Complete type annotations are not needed byt added to make the conversions explicitly visible.*
    ///
    /// ```
    /// use orx_imp_vec::prelude::*;
    ///
    /// let imp: ImpVec<char, SplitVec<char, Doubling>> = SplitVec::with_doubling_growth(2).into();
    /// for _ in 0..3 {
    ///     imp.push('s');
    /// }
    /// assert_eq!(imp, vec!['s', 's', 's']);
    ///
    /// let split: SplitVec<char, Doubling> = imp.into_pinned();
    /// assert_eq!(split, vec!['s', 's', 's']);
    ///
    /// let imp_again: ImpVec<_, _> = split.into();
    /// imp_again.push('s');
    /// assert_eq!(imp_again, vec!['s', 's', 's', 's']);
    ///
    /// let split_back = imp_again.into_pinned();
    /// assert_eq!(split_back, vec!['s', 's', 's', 's']);
    /// ```
    pub fn into_pinned(self) -> P {
        self.cell.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn into_split_vec() {
        fn test<G: Growth>(mut pinned_vec: SplitVec<usize, G>) {
            for i in 0..400 {
                pinned_vec.push(i * 2);
            }

            let imp: ImpVec<_, _> = pinned_vec.into();
            for i in 400..1000 {
                imp.push(i * 2);
            }

            let expected: Vec<_> = (0..1000).map(|i| i * 2).collect();

            assert_eq!(expected, imp);

            let pinned_back: SplitVec<usize, G> = imp.into();
            assert_eq!(expected, pinned_back);
        }

        test_all_growth_types!(test);
    }
}
