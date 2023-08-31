use orx_pinned_vec::{NotSelfRefVecItem, PinnedVecSimple};

use crate::ImpVec;

impl<T, P> ImpVec<T, P>
where
    P: PinnedVecSimple<T>,
    T: NotSelfRefVecItem,
{
    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    ///
    /// # Safety
    ///
    /// `insert` operation for an `ImpVec` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    #[inline(always)]
    pub fn insert(&mut self, index: usize, element: T) {
        self.cell.borrow_mut().insert(index, element)
    }
    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Safety
    ///
    /// `remove` operation for a `ImpVec` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> T {
        self.cell.borrow_mut().remove(index)
    }
    /// Removes the last element from a vector and returns it, or None if it is empty.
    ///
    /// # Safety
    ///
    /// `pop` operation for a `ImpVec` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.cell.borrow_mut().pop()
    }
}
