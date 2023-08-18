//! `ImpVec` stands for 'immutable-push-vec'.
//! It uses `orx_split_vec::SplitVec` as the underlying data structure,
//! and additionally allows for push/extend operations with an immutable reference.

#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod deref;
mod eq;
mod extend;
mod imp_vec;
mod new_imp_vec;
mod push;

pub use self::imp_vec::ImpVec;
