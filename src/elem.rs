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
}
