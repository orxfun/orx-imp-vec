use core::marker::PhantomData;

pub struct Elem<'a, T> {
    ptr: *const T,
    vec_ref: PhantomData<fn() -> &'a ()>,
}
