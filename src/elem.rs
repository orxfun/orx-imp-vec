use crate::ImpVec;
use core::marker::PhantomData;
use orx_fixed_vec::PinnedVec;

pub struct Elem<'a, T> {
    ptr: *mut T,
    vec_ref: &'a (),
}

impl<'a, T> Clone for Elem<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Elem<'a, T> {}

impl<'a, T> Elem<'a, T> {
    pub(crate) fn new_from_pushed<P>(vec: &'a ImpVec<T, P>, value: T) -> Self
    where
        P: PinnedVec<T>,
    {
        let ptr = vec.imp_push_get_ptr(value) as *mut T;
        let vec_ref = &();
        Self { ptr, vec_ref }
    }

    pub(crate) fn new_from_position<P>(vec: &'a ImpVec<T, P>, index: usize) -> Option<Self>
    where
        P: PinnedVec<T>,
    {
        vec.pinned().get(index).map(|x| {
            let ptr = x as *const T as *mut T;
            let vec_ref = &();
            Self { ptr, vec_ref }
        })
    }

    /// # SAFETY
    ///
    /// It is safe to use `get` method provided that:
    ///
    /// * obtained reference is used only on the same thread that owns the parent `ImpVec`,
    ///   and never sent to another thread.
    ///
    /// ## Safety Details
    ///
    /// `ptr` is guarenteed to be valid by two `new_` methods.
    /// ImpVec holding `ptr` is alive for `'a`.
    /// ImpVec is backed by a pinned vector and it is grow only.
    /// Hence, `ptr` is still valid.
    /// Further, ImpVec is not Sync, and hence, there is no race condition.
    ///
    /// However, one can create a `&T` reference with `get`;
    /// send the reference to another thread,
    /// and change its value using `set` on this thread.
    /// This leads to a race condition.
    #[inline(always)]
    pub unsafe fn get(self) -> &'a T {
        unsafe { &*self.ptr }
    }

    pub fn cloned(self) -> T
    where
        T: Clone,
    {
        // SAFETY: this call does not allow to send `&T` ref to another thread
        unsafe { self.get() }.clone()
    }

    pub fn copied(self) -> T
    where
        T: Copy,
    {
        // SAFETY: this call does not allow to send `&T` ref to another thread
        *unsafe { self.get() }
    }

    pub fn map<F, U>(self, f: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        // TODO: explain safety guarantees
        f(unsafe { self.get() })
    }

    pub fn replace(self, value: T) -> T {
        // SAFETY: `ptr` is guarenteed to be valid by two `new_` methods.
        // ImpVec holding `ptr` is alive for `'a`.
        // ImpVec is backed by a pinned vector and it is grow only.
        // Hence, `ptr` is still valid.
        // Further, ImpVec is not Sync, and hence, there is no race condition.
        unsafe { self.ptr.replace(value) }
    }

    pub fn set(self, value: T) {
        _ = self.replace(value);
    }

    pub fn update<F>(self, f: F)
    where
        F: FnOnce(&mut T),
    {
        // TODO: explain safety guarantees
        f(unsafe { &mut *self.ptr });
    }

    pub fn position_in<P>(self, vec: &'a ImpVec<T, P>) -> Option<usize>
    where
        P: PinnedVec<T>,
    {
        // SAFETY: reference does not leak to another thread
        let rf = unsafe { self.get() };
        vec.pinned().index_of(rf)
    }

    pub fn as_ptr(self) -> *const T {
        self.ptr
    }
}
