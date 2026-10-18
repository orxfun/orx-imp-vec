use crate::ImpVec;
use core::marker::PhantomData;
use orx_fixed_vec::PinnedVec;

pub struct Elem<'a, T> {
    ptr: *const T,
    vec_ref: PhantomData<fn() -> &'a ()>,
}

impl<'a, T> Elem<'a, T> {
    pub(crate) fn new_from_pushed<P>(vec: &ImpVec<T, P>, value: T) -> Self
    where
        P: PinnedVec<T>,
    {
        let ptr = vec.imp_push_get_ptr(value);
        let vec_ref = PhantomData;
        Self { ptr, vec_ref }
    }

    pub(crate) fn new_from_position<P>(vec: &ImpVec<T, P>, index: usize) -> Option<Self>
    where
        P: PinnedVec<T>,
    {
        vec.get(index).map(|x| {
            let ptr = x as *const T;
            let vec_ref = PhantomData;
            Self { ptr, vec_ref }
        })
    }

    pub fn get(&self) -> &'a T {
        // SAFETY: `ptr` is guarenteed to be valid by two `new_` methods.
        // ImpVec holding `ptr` is alive for `'a`.
        // ImpVec is backed by a pinned vector and it is grow only.
        // Hence, `ptr` is still valid.
        // Further, ImpVec is not Sync, and hence, there is no race condition.
        unsafe { &*self.ptr }
    }

    pub fn set(&self, value: T) {}
}
