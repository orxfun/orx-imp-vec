//! # orx-imp-vec
//!
//! [![orx-imp-vec crate](https://img.shields.io/crates/v/orx-imp-vec.svg)](https://crates.io/crates/orx-imp-vec)
//! [![orx-imp-vec documentation](https://docs.rs/orx-imp-vec/badge.svg)](https://docs.rs/orx-imp-vec)
//!
//! `ImpVec`, standing for immutable push vector 👿, is a data structure which allows appending elements with a shared reference.
//!
//! Specifically, it extends vector capabilities with the following two methods:
//! * `fn imp_push(&self, value: T)`
//! * `fn imp_extend_from_slice(&self, slice: &[T])`
//!
//! Note that both of these methods can be called with `&self` rather than `&mut self`.
//!
//! # Motivation
//!
//! Appending to a vector with a shared reference sounds unconventional, and it is. However, if we consider our vector as a bag of or a container of things rather than having a collective meaning; then, appending element or elements to the end of the vector:
//! * does not mutate any of already added elements, and hence,
//! * **it is not different than creating a new element in the scope**.
//!
//! # Safety
//!
//! It is natural to expect that appending elements to a vector does not affect already added elements. However, this is usually not the case due to underlying memory management. For instance, `std::vec::Vec` may move already added elements to different memory locations to maintain the contagious layout of the vector.
//!
//! [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) prevents such implicit changes in memory locations. It guarantees that push and extend methods keep memory locations of already added elements intact. Therefore, it is perfectly safe to hold on to references of the vector while appending elements.
//!
//! Consider the classical example that does not compile, which is often presented to highlight the safety guarantees of rust:
//!
//! ```rust
//! let mut vec = vec![0, 1, 2, 3];
//!
//! let ref_to_first = &vec[0];
//! assert_eq!(ref_to_first, &0);
//!
//! vec.push(4);
//!
//! // does not compile due to the following reason:  cannot borrow `vec` as mutable because it is also borrowed as immutable
//! // assert_eq!(ref_to_first, &0);
//! ```
//!
//! This beloved feature of the borrow checker of rust is not required and used for `imp_push` and `imp_extend_from_slice` methods of `ImpVec` since these methods do not require a `&mut self` reference. Therefore, the following code compiles and runs perfectly safely.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! let mut vec = ImpVec::new();
//! vec.extend_from_slice(&[0, 1, 2, 3]);
//!
//! let ref_to_first = &vec[0];
//! assert_eq!(ref_to_first, &0);
//!
//! vec.imp_push(4);
//! assert_eq!(vec.len(), 5);
//!
//! vec.imp_extend_from_slice(&[6, 7]);
//! assert_eq!(vec.len(), 7);
//!
//! assert_eq!(ref_to_first, &0);
//! ```
//!
//! ## Contributing
//!
//! Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-imp-vec/issues/new) or create a PR.
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

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

#[cfg(test)]
mod tests;

mod common_traits;
mod imp_vec;
mod new;
/// Common traits, structs, enums.
pub mod prelude;

pub use imp_vec::ImpVec;
