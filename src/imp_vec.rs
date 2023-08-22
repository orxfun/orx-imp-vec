use orx_split_vec::{Fragment, LinearGrowth, SplitVec, SplitVecGrowth};
use std::cell::RefCell;

#[derive(Debug)]
/// `ImpVec` stands for 'immutable-push-vec'.
///
/// It uses `orx_split_vec::SplitVec` as the underlying data structure,
/// and hence, has the following features:
///
/// * Flexible in growth strategies; custom strategies can be defined.
/// * Growth does not cause any memory copies.
/// * Capacity of an already created fragment is never changed.
/// * The above feature allows the data to stay pinned in place. Memory location of an item added to the split vector will never change unless it is removed from the vector or the vector is dropped.
///
/// In addition, it allows to push/extend the vector with an immutable reference.
///
/// This allows to hold on the references of already pushed elements
/// while building the collection.
pub struct ImpVec<T, G = LinearGrowth>
where
    G: SplitVecGrowth<T>,
{
    pub(crate) split_vec: RefCell<SplitVec<T, G>>,
}

impl<T, G> ImpVec<T, G>
where
    G: SplitVecGrowth<T>,
{
    pub(crate) fn as_mut_ptr(&self) -> *mut SplitVec<T, G> {
        self.split_vec.as_ptr()
    }
    pub(crate) fn add_fragment(&self) {
        let new_fragment = {
            let capacity = self.growth.new_fragment_capacity(self.fragments());
            Fragment::new(capacity)
        };
        unsafe {
            self.split_vec
                .borrow_mut()
                .fragments_mut()
                .push(new_fragment);
        }
    }
    pub(crate) fn add_fragment_with_first_value(&self, first_value: T) {
        let new_fragment = {
            let capacity = self.growth.new_fragment_capacity(self.fragments());
            Fragment::new_with_first_value(capacity, first_value)
        };
        unsafe {
            self.split_vec
                .borrow_mut()
                .fragments_mut()
                .push(new_fragment);
        }
    }
    pub(crate) fn has_capacity_for_one(&self) -> bool {
        self.fragments()
            .last()
            .map(|f| f.has_capacity_for_one())
            .unwrap_or(false)
    }
}
