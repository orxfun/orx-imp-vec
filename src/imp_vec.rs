use core::{cell::UnsafeCell, marker::PhantomData};
use orx_pinned_vec::PinnedVec;
use orx_split_vec::SplitVec;

/// `ImpVec`, stands for immutable push vector 👿, is a data structure which allows appending elements with a shared reference.
///
/// Specifically, it extends vector capabilities with the following two methods:
/// * `fn imp_push(&self, value: T)`
/// * `fn imp_extend_from_slice(&self, slice: &[T])`
///
/// Note that both of these methods can be called with `&self` rather than `&mut self`.
///
/// # Motivation
///
/// Appending to a vector with a shared reference sounds unconventional, and it is.
/// However, if we consider our vector as a bag of or a container of things rather than having a collective meaning;
/// then, appending element or elements to the end of the vector:
/// * does not mutate any of already added elements, and hence,
/// * **it is not different than creating a new element in the scope**.
///
/// # Safety
///
/// It is natural to expect that appending elements to a vector does not affect already added elements.
/// However, this is usually not the case due to underlying memory management.
/// For instance, `std::vec::Vec` may move already added elements to different memory locations to maintain the contagious layout of the vector.
///
/// `PinnedVec` prevents such implicit changes in memory locations.
/// It guarantees that push and extend methods keep memory locations of already added elements intact.
/// Therefore, it is perfectly safe to hold on to references of the vector while appending elements.
///
/// Consider the classical example that does not compile, which is often presented to highlight the safety guarantees of rust:
///
/// ```rust
/// let mut vec = vec![0, 1, 2, 3];
///
/// let ref_to_first = &vec[0];
/// assert_eq!(ref_to_first, &0);
///
/// vec.push(4);
///
/// // does not compile due to the following reason:  cannot borrow `vec` as mutable because it is also borrowed as immutable
/// // assert_eq!(ref_to_first, &0);
/// ```
///
/// This wonderful feature of the borrow checker of rust is not required and used for `imp_push` and `imp_extend_from_slice` methods of `ImpVec`
/// since these methods do not require a `&mut self` reference.
/// Therefore, the following code compiles and runs perfectly safely.
///
/// ```rust
/// use orx_imp_vec::*;
///
/// let mut vec = ImpVec::new();
/// vec.extend_from_slice(&[0, 1, 2, 3]);
///
/// let ref_to_first = &vec[0];
/// assert_eq!(ref_to_first, &0);
///
/// vec.imp_push(4);
/// assert_eq!(vec.len(), 5);
///
/// vec.imp_extend_from_slice(&[6, 7]);
/// assert_eq!(vec.len(), 7);
///
/// assert_eq!(ref_to_first, &0);
/// ```
pub struct ImpVec<T, P = SplitVec<T>>
where
    P: PinnedVec<T>,
{
    pub(crate) pinned_vec: UnsafeCell<P>,
    pub(crate) phantom: PhantomData<T>,
}

impl<T, P: PinnedVec<T>> ImpVec<T, P> {
    /// Consumes the imp-vec into the wrapped inner pinned vector.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_split_vec::SplitVec;
    /// use orx_imp_vec::ImpVec;
    ///
    /// let pinned_vec = SplitVec::new();
    ///
    /// let imp_vec = ImpVec::from(pinned_vec);
    /// imp_vec.imp_push(42);
    ///
    /// let pinned_vec = imp_vec.into_inner();
    /// assert_eq!(&pinned_vec, &[42]);
    /// ```
    pub fn into_inner(self) -> P {
        self.pinned_vec.into_inner()
    }

    /// Pushes the `value` to the vector.
    /// This method differs from the `push` method with the required reference.
    /// Unlike `push`, `imp_push` allows to push the element with a shared reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let mut vec = ImpVec::new();
    ///
    /// // regular push with &mut self
    /// vec.push(42);
    ///
    /// // hold on to a reference to the first element
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &42);
    ///
    /// // imp_push with &self
    /// vec.imp_push(7);
    ///
    /// // due to `PinnedVec` guarantees, this push will never invalidate prior references
    /// assert_eq!(ref_to_first, &42);
    /// ```
    ///
    /// # Safety
    ///
    /// Wrapping a `PinnedVec` with an `ImpVec` provides with two additional methods: `imp_push` and `imp_extend_from_slice`.
    /// Note that these push and extend methods grow the vector by appending elements to the end.
    ///
    /// It is natural to expect that these operations do not change the memory locations of already added elements.
    /// However, this is usually not the case due to underlying allocations.
    /// For instance, `std::vec::Vec` may move already added elements in memory to maintain the contagious layout of the vector.
    ///
    /// `PinnedVec` prevents such implicit changes in memory locations.
    /// It guarantees that push and extend methods keep memory locations of already added elements intact.
    /// Therefore, it is perfectly safe to hold on to references of the vector while appending elements.
    ///
    /// Consider the classical example that does not compile, which is often presented to highlight the safety guarantees of rust:
    ///
    /// ```rust
    /// let mut vec = vec![0, 1, 2, 3];
    ///
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &0);
    ///
    /// vec.push(4);
    ///
    /// // does not compile due to the following reason:  cannot borrow `vec` as mutable because it is also borrowed as immutable
    /// // assert_eq!(ref_to_first, &0);
    /// ```
    ///
    /// This wonderful feature of the borrow checker of rust is not required and used for `imp_push` and `imp_extend_from_slice` methods of `ImpVec`
    /// since these methods do not require a `&mut self` reference.
    /// Therefore, the following code compiles and runs perfectly safely.
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let mut vec = ImpVec::new();
    /// vec.extend_from_slice(&[0, 1, 2, 3]);
    ///
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &0);
    ///
    /// vec.imp_push(4);
    /// assert_eq!(vec.len(), 5);
    ///
    /// assert_eq!(ref_to_first, &0);
    /// ```
    ///
    /// Although unconventional, this makes sense when we consider the `ImpVec` as a bag or container of things, rather than having a collective meaning.
    /// In other words, when we do not rely on reduction methods, such as `count` or `sum`, appending element or elements to the end of the vector:
    /// * does not mutate any of already added elements, and hence,
    /// * **it is not different than creating a new element in the scope**.
    pub fn imp_push(&self, value: T) {
        self.pinned_mut().push(value);
    }

    /// Pushes the `value` to the vector and returns a reference to it.
    ///
    /// It is the composition of [`vec.imp_push(value)`] call followed by `&vec[vec.len() - 1]`.
    ///
    /// [`vec.imp_push(value)`]: crate::ImpVec::imp_push
    ///
    /// # Examples
    ///
    /// This method provides a shorthand for the following common use case.
    ///
    /// ```
    /// use orx_imp_vec::*;
    ///
    /// let vec = ImpVec::new();
    ///
    /// vec.imp_push('a');
    /// let a = &vec[vec.len() - 1];
    /// assert_eq!(a, &'a');
    ///
    /// // or with imp_push_get_ref
    ///
    /// let b = vec.imp_push_get_ref('b');
    /// assert_eq!(b, &'b');
    /// ```
    pub fn imp_push_get_ref(&self, value: T) -> &T {
        let pinned = self.pinned_mut();
        pinned.push(value);
        &pinned[pinned.len() - 1]
    }

    /// Extends the vector with the given `slice`.
    /// This method differs from the `extend_from_slice` method with the required reference.
    /// Unlike `extend_from_slice`, `imp_extend_from_slice` allows to push the element with a shared reference.
    ///
    /// # Example
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let mut vec = ImpVec::new();
    ///
    /// // regular extend_from_slice with &mut self
    /// vec.extend_from_slice(&[42]);
    ///
    /// // hold on to a reference to the first element
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &42);
    ///
    /// // imp_extend_from_slice with &self
    /// vec.imp_extend_from_slice(&[0, 1, 2, 3]);
    /// assert_eq!(vec.len(), 5);
    ///
    /// // due to `PinnedVec` guarantees, this extend will never invalidate prior references
    /// assert_eq!(ref_to_first, &42);
    /// ```
    ///
    /// # Safety
    ///
    /// Wrapping a `PinnedVec` with an `ImpVec` provides with two additional methods: `imp_push` and `imp_extend_from_slice`.
    /// Note that these push and extend methods grow the vector by appending elements to the end.
    ///
    /// It is natural to expect that these operations do not change the memory locations of already added elements.
    /// However, this is usually not the case due to underlying allocations.
    /// For instance, `std::vec::Vec` may move already added elements in memory to maintain the contagious layout of the vector.
    ///
    /// `PinnedVec` prevents such implicit changes in memory locations.
    /// It guarantees that push and extend methods keep memory locations of already added elements intact.
    /// Therefore, it is perfectly safe to hold on to references of the vector while appending elements.
    ///
    /// Consider the classical example that does not compile, which is often presented to highlight the safety guarantees of rust:
    ///
    /// ```rust
    /// let mut vec = vec![0];
    ///
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &0);
    ///
    /// vec.extend_from_slice(&[1, 2, 3, 4]);
    ///
    /// // does not compile due to the following reason:  cannot borrow `vec` as mutable because it is also borrowed as immutable
    /// // assert_eq!(ref_to_first, &0);
    /// ```
    ///
    /// This wonderful feature of the borrow checker of rust is not required and used for `imp_push` and `imp_extend_from_slice` methods of `ImpVec`
    /// since these methods do not require a `&mut self` reference.
    /// Therefore, the following code compiles and runs perfectly safely.
    ///
    /// ```rust
    /// use orx_imp_vec::*;
    ///
    /// let mut vec = ImpVec::new();
    /// vec.push(0);
    ///
    /// let ref_to_first = &vec[0];
    /// assert_eq!(ref_to_first, &0);
    ///
    /// vec.imp_extend_from_slice(&[1, 2, 3, 4]);
    ///
    /// assert_eq!(ref_to_first, &0);
    /// ```
    ///
    /// Although unconventional, this makes sense when we consider the `ImpVec` as a bag or container of things, rather than having a collective meaning.
    /// In other words, when we do not rely on reduction methods, such as `count` or `sum`, appending element or elements to the end of the vector:
    /// * does not mutate any of already added elements, and hence,
    /// * **it is not different than creating a new element in the scope**.
    pub fn imp_extend_from_slice(&self, slice: &[T])
    where
        T: Clone,
    {
        self.pinned_mut().extend_from_slice(slice);
    }

    // helper
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn pinned_mut(&self) -> &mut P {
        // SAFETY: `ImpVec` does not implement Send or Sync.
        // Further `imp_push` and `imp_extend_from_slice` methods are safe to call with a shared reference due to pinned vector guarantees.
        // All other calls to this internal method require a mutable reference.
        unsafe { &mut *self.pinned_vec.get() }
    }
}
