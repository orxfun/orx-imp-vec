use crate::ImpVec;
use orx_split_vec::{SplitVec, SplitVecGrowth};
use std::cell::RefCell;

// into ImpVec
impl<T, G> From<SplitVec<T, G>> for ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
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
    fn from(value: SplitVec<T, G>) -> Self {
        Self {
            split_vec: RefCell::new(value),
        }
    }
}
impl<T, G> From<Vec<T>> for ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
    SplitVec<T, G>: From<Vec<T>>,
{
    /// Converts a `Vec` into an `ImpVec` by first moving the vector
    /// into the underlying split vector as the first fragment
    /// without copying the data, and then converting the `SplitVec`
    /// into `ImpVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let vec = vec!['a', 'b', 'c'];
    /// let vec_capacity = vec.capacity();
    ///
    /// let split_vec: ImpVec<_> = vec.into();
    ///
    /// assert_eq!(split_vec, &['a', 'b', 'c']);
    /// assert_eq!(1, split_vec.fragments().len());
    /// assert_eq!(vec_capacity, split_vec.fragments()[0].capacity());
    /// ```
    fn from(value: Vec<T>) -> Self {
        Self {
            split_vec: RefCell::new(value.into()),
        }
    }
}

// from ImpVec
impl<T, G> From<ImpVec<T, G>> for SplitVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Converts an `ImpVec` into a `SplitVec` by simply
    /// moving out the split vector from the imp-vector
    /// without copying the data.
    ///
    /// Growth strategy is preserved.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    /// use orx_split_vec::{SplitVec, LinearGrowth};
    ///
    /// let imp_vec = ImpVec::with_linear_growth(10);
    /// imp_vec.push(0);
    /// imp_vec.push(1);
    ///
    /// let mut split_vec: SplitVec<_, LinearGrowth> = imp_vec.into();
    /// assert_eq!(split_vec, &[0, 1]);
    ///
    /// split_vec.push(2);
    /// assert_eq!(split_vec, &[0, 1, 2]);
    /// ```
    fn from(value: ImpVec<T, G>) -> Self {
        value.split_vec.into_inner()
    }
}

impl<T, G> From<ImpVec<T, G>> for Vec<T>
where
    G: SplitVecGrowth<T>,
{
    /// Converts the `ImpVec` into a standard `std::vec::Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let imp_vec = ImpVec::with_linear_growth(4);
    /// imp_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, imp_vec.fragments().len());
    ///
    /// let vec: Vec<_> = imp_vec.into();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let imp_vec = ImpVec::with_linear_growth(4);
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
    fn from(value: ImpVec<T, G>) -> Self {
        let split_vec: SplitVec<_, G> = value.into();
        split_vec.into()
    }
}

impl<T, G> ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    /// Converts the `ImpVec` into a standard `Vec` with a contagious memory layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use orx_imp_vec::ImpVec;
    ///
    /// let imp_vec = ImpVec::with_linear_growth(4);
    /// imp_vec.extend_from_slice(&['a', 'b', 'c']);
    ///
    /// assert_eq!(1, imp_vec.fragments().len());
    ///
    /// let vec = imp_vec.to_vec();
    /// assert_eq!(vec, &['a', 'b', 'c']);
    ///
    /// let imp_vec = ImpVec::with_linear_growth(4);
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
