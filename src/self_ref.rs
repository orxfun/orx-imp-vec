use crate::ImpVec;
use orx_pinned_vec::{PinnedVec, SelfRefVecItem};

impl<'a, T, P> ImpVec<T, P>
where
    P: PinnedVec<T> + 'a,
    T: SelfRefVecItem<'a> + 'a,
{
    /// Sets the `prev` reference of the element at the `idx` to the
    /// element at the `prev_idx`.
    ///
    /// If `prev_idx.is none()` the `prev` reference will be set to `None`.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds;
    /// or if `prev_idx.is_some()` and the underlying id is out of bounds.
    ///
    /// # Safety
    ///
    /// The call is trivially safe when `prev_idx.is_none()` since the optional
    /// reference is just set to `None`.
    ///
    /// Otherwise, the method call is safe due to the following reasons:
    ///
    /// * Due to bounds-checks both the main element and the target element
    /// that the main element will hold a reference of belong to the vector;
    /// in other words, the main element will be holding a reference
    /// to another element in the same vector.
    /// * Since `ImpVec` wraps a `PinnedVec`, memory location of the target
    /// element cannot be change while the vector is growing.
    /// * The methods which can possibly change the memory location of the
    /// target element such as `remove`, `pop`, `swap`, `truncate` or `insert`
    /// are `unsafe` for `PinnedVec`s, and hence, for `ImpVec`s.
    /// * The only safe method which could change the memory location of the
    /// target element is `clear` method. This is safe since it would drop
    /// the main element at the same time together with its reference and
    /// target element.
    ///
    /// Due to these guarantees, the built up inter-elements reference is
    /// and will remain valid.
    pub fn set_prev(&mut self, idx: usize, prev_idx: Option<usize>) {
        let node = unsafe { self.get_mut(idx) }.expect("out-of-bounds");
        let prev_node = prev_idx.map(|idx| unsafe { self.get_ref(idx) }.expect("out-of-bounds"));
        node.set_prev(prev_node);
    }
    /// Sets the `next` reference of the element at the `idx` to the
    /// element at the `next_idx`.
    ///
    /// If `next_idx.is none()` the `prev` reference will be set to `None`.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds;
    /// or if `prev_idx.is_some()` and the underlying id is out of bounds.
    ///
    /// # Safety
    ///
    /// The call is trivially safe when `next_idx.is_none()` since the optional
    /// reference is just set to `None`.
    ///
    /// Otherwise, the method call is safe due to the following reasons:
    ///
    /// * Due to bounds-checks both the main element and the target element
    /// that the main element will hold a reference of belong to the vector;
    /// in other words, the main element will be holding a reference
    /// to another element in the same vector.
    /// * Since `ImpVec` wraps a `PinnedVec`, memory location of the target
    /// element cannot be change while the vector is growing.
    /// * The methods which can possibly change the memory location of the
    /// target element such as `remove`, `pop`, `swap`, `truncate` or `insert`
    /// are `unsafe` for `PinnedVec`s, and hence, for `ImpVec`s.
    /// * The only safe method which could change the memory location of the
    /// target element is `clear` method. This is safe since it would drop
    /// the main element at the same time together with its reference and
    /// target element.
    ///
    /// Due to these guarantees, the built up inter-elements reference is
    /// and will remain valid.
    pub fn set_next(&mut self, idx: usize, next_idx: Option<usize>) {
        let node = unsafe { self.get_mut(idx) }.expect("out-of-bounds");
        let next_node = next_idx.map(|idx| unsafe { self.get_ref(idx) }.expect("out-of-bounds"));
        node.set_next(next_node);
    }
}
