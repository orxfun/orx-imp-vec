#![doc = include_str!("../README.md")]
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
#![no_std]

extern crate alloc;

mod common_traits;
mod imp_vec;
mod new;

pub use imp_vec::ImpVec;
pub use orx_fixed_vec::FixedVec;
pub use orx_pinned_vec::PinnedVec;
pub use orx_split_vec::{Doubling, Growth, Linear, Recursive, SplitVec};
