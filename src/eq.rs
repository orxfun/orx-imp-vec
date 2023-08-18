use crate::ImpVec;
use orx_split_vec::SplitVec;

impl<T: PartialEq, U> PartialEq<U> for ImpVec<T>
where
    U: AsRef<[T]>,
{
    fn eq(&self, other: &U) -> bool {
        &self.split_vec.borrow() as &SplitVec<T> == other
    }
}
impl<T: PartialEq> PartialEq<ImpVec<T>> for ImpVec<T> {
    fn eq(&self, other: &ImpVec<T>) -> bool {
        &self.split_vec.borrow() as &SplitVec<T> == &other.split_vec.borrow() as &SplitVec<T>
    }
}
impl<T: PartialEq> Eq for ImpVec<T> {}
