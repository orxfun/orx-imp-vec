use crate::ImpVec;
use orx_pinned_vec::PinnedVec;

/// Iterator over the `ImpVec`.
///
/// Temporary naive implementation.
#[derive(Debug, Clone)]
pub struct ImpVecIter<'a, T, P>
where
    P: PinnedVec<T>,
{
    pub(crate) vec: &'a ImpVec<T, P>,
    pub(crate) i: usize,
}
/// Iterator over the `ImpVec`.
///
/// Temporary naive implementation.
#[derive(Debug)]
pub struct ImpVecIterMut<'a, T, P>
where
    P: PinnedVec<T>,
{
    pub(crate) vec: &'a mut ImpVec<T, P>,
    pub(crate) i: usize,
}

impl<'a, T, P> Iterator for ImpVecIter<'a, T, P>
where
    P: PinnedVec<T>,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let data = self.vec.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            let i = self.i;
            self.i += 1;
            pinned_vec.get(i)
        }
    }
}
impl<'a, T, P> Iterator for ImpVecIterMut<'a, T, P>
where
    P: PinnedVec<T>,
{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let data = self.vec.as_mut_ptr();
        unsafe {
            let pinned_vec = &mut *data;
            let i = self.i;
            self.i += 1;
            pinned_vec.get_mut(i)
        }
    }
}

impl<T, P> ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    /// Returns an iterator for the imp-vec.
    pub fn iter(&self) -> ImpVecIter<'_, T, P> {
        ImpVecIter { vec: self, i: 0 }
    }
    /// Returns a mutable iterator for the imp-vec.
    pub fn iter_mut(&mut self) -> ImpVecIterMut<'_, T, P> {
        ImpVecIterMut { vec: self, i: 0 }
    }
}

impl<'a, T, P> IntoIterator for &'a ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    type Item = &'a T;
    type IntoIter = ImpVecIter<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        ImpVecIter { vec: self, i: 0 }
    }
}
impl<'a, T, P> IntoIterator for &'a mut ImpVec<T, P>
where
    P: PinnedVec<T>,
{
    type Item = &'a mut T;
    type IntoIter = ImpVecIterMut<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        ImpVecIterMut { vec: self, i: 0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test_all_growth_types;

    #[test]
    fn iter_mut() {
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            for i in 0..100 {
                imp.push(i);
            }

            for x in &mut imp {
                *x += 42;
            }

            for x in imp.iter_mut() {
                *x += 42;
            }

            for (i, x) in imp.iter().enumerate() {
                assert_eq!(i + 42 + 42, *x);
            }
        }

        test_all_growth_types!(test);
    }

    #[test]
    fn into_iter() {
        fn get_into_iter<'a, P>(imp: &'a ImpVec<usize, P>)
        where
            P: PinnedVec<usize> + 'a,
        {
            let iter = imp.into_iter();
            for (i, x) in iter.enumerate() {
                assert_eq!(i, *x);
            }
        }
        fn test<P: PinnedVec<usize>>(pinned_vec: P) {
            let imp: ImpVec<_, _> = pinned_vec.into();
            for i in 0..841 {
                imp.push(i);
            }
            get_into_iter(&imp);
        }

        test_all_growth_types!(test);
    }
}
