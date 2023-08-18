use crate::ImpVec;
use orx_split_vec::SplitVec;
use std::cell::RefCell;

impl<T> From<SplitVec<T>> for ImpVec<T> {
    /// Converts a `SplitVec` into a `ImpVec` by
    /// moving the split-vector into the imp-vector,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    /// use orx_split_vec::SplitVec;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let mut split_vec = SplitVec::default(); // required mut to push
    /// split_vec.push(0);
    /// split_vec.push(1);
    ///
    /// let imp_vec: ImpVec<_> = split_vec.into(); // can push w/o mut ref
    /// assert_eq!(imp_vec, &[0, 1]);
    ///
    /// imp_vec.push(2);
    ///assert_eq!(imp_vec, &[0, 1, 2]);
    /// ```
    fn from(value: SplitVec<T>) -> Self {
        Self {
            split_vec: RefCell::new(value),
        }
    }
}

impl<T> From<ImpVec<T>> for SplitVec<T> {
    /// Converts a `ImpVec` into a `SplitVec` by
    /// moving out the split vector from the imp-vector,
    /// without copying the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    /// use orx_split_vec::SplitVec;
    ///
    /// let imp_vec = ImpVec::default();
    /// imp_vec.push(0);
    /// imp_vec.push(1);
    ///
    /// let mut split_vec: SplitVec<_> = imp_vec.into();
    /// assert_eq!(split_vec, &[0, 1]);
    ///
    /// split_vec.push(2);
    /// assert_eq!(split_vec, &[0, 1, 2]);
    /// ```
    fn from(value: ImpVec<T>) -> Self {
        value.split_vec.into_inner()
    }
}

impl<T> From<Vec<T>> for ImpVec<T> {
    /// Converts a `Vec` into a `ImpVec` by
    /// moving the vector into the split vector as the first fragment,
    /// without copying the data,
    /// and converting the split vector into an imp-vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let imp_vec: ImpVec<_> = vec.into();
    ///
    /// assert_eq!(imp_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, imp_vec.fragments().len());
    /// assert_eq!(vec_capacity, imp_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        let split_vec: SplitVec<_> = value.into();
        split_vec.into()
    }
}

impl<T> From<ImpVec<T>> for Vec<T> {
    /// Converts the `ImpVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    /// use orx_split_vec::FragmentGrowth;
    ///
    /// let imp_vec = ImpVec::with_growth(FragmentGrowth::constant(4));
    /// imp_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, imp_vec.fragments().len());
    ///
    /// let vec: Vec<_> = imp_vec.into();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let imp_vec = ImpVec::with_growth(FragmentGrowth::constant(4));
    /// for i in 0..10 {
    ///     imp_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], imp_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], imp_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], imp_vec.fragments()[2].as_slice());
    ///
    /// let vec: Vec<_> = imp_vec.into();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    fn from(value: ImpVec<T>) -> Self {
        let split_vec: SplitVec<_> = value.into();
        split_vec.into()
    }
}

impl<T> ImpVec<T> {
    /// Converts the `ImpVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    /// use orx_split_vec::FragmentGrowth;
    ///
    /// let imp_vec = ImpVec::with_growth(FragmentGrowth::constant(4));
    /// imp_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, imp_vec.fragments().len());
    ///
    /// let vec = imp_vec.to_vec();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let imp_vec = ImpVec::with_growth(FragmentGrowth::constant(4));
    /// for i in 0..10 {
    ///     imp_vec.push(i);
    /// }
    /// assert_eq!(&[0, 1, 2, 3], imp_vec.fragments()[0].as_slice());
    /// assert_eq!(&[4, 5, 6, 7], imp_vec.fragments()[1].as_slice());
    /// assert_eq!(&[8, 9], imp_vec.fragments()[2].as_slice());
    ///
    /// let vec = imp_vec.to_vec();
    /// assert_eq!(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec.as_slice());
    /// ```
    pub fn to_vec(self) -> Vec<T> {
        self.into()
    }
}
