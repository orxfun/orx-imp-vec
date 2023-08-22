//! # orx-imp-vec
//!
//! An `ImpVec` uses [`SplitVec`](https://crates.io/crates/orx-split-vec) as the underlying data model,
//! and hence, inherits the following features:
//!
//! * vector growth does not require memory copies,
//! * therefore, growth does not cause the memory locations of elements to change, and
//! * provides flexible strategies to explicitly define how the vector should grow.
//!
//! Additionally, `ImpVec` allows to push to or extend the vector with an immutable reference;
//! hence, it gets the name `ImpVec`:
//!
//! * imp-vec stands for 'immutable push vector',
//! * and also hints for the little evil behavior it has.
//!
//! ## Safety-1
//!
//! Pushing to a vector with an immutable reference sounds unsafe;
//! however, `ImpVec` provides the safety guarantees.
//!
//! Consider the following example using `std::vec::Vec` which does not compile:
//!
//! ```rust
//! let mut vec = Vec::with_capacity(2);
//! vec.extend_from_slice(&[0, 1]);
//!
//! let ref0 = &vec[0];
//! vec.push(2);
//! // let value0 = *ref0; // does not compile!
//! ```
//!
//! Why does `push` invalidate the reference to the first element which is already pushed to the vector?
//! * the vector has a capacity of 2; and hence, the push will lead to an expansion of the vector's capacity;
//! * it is possible that the underlying data will be copied to another place in memory;
//! * in this case `ref0` will be an invalid reference and dereferencing it would lead to UB.

//! However, `ImpVec` uses the `SplitVec` as its underlying data model
//! which guarantees that the memory location of an item added to the split vector will never change
//! unless it is removed from the vector or the vector is dropped.
//!
//! Therefore, the  following `ImpVec` version compiles and preserves the validity of the references.
//!
//! ```rust
//! use orx_imp_vec::ImpVec;
//!
//! let vec = ImpVec::with_doubling_growth(2);
//! vec.extend_from_slice(&[0, 1]);
//!
//! let ref0 = &vec[0];
//! let ref0_addr = ref0 as *const i32; // address before growth
//!
//! vec.push(2); // capacity is increased here
//!
//! let ref0_addr_after_growth = &vec[0] as *const i32; // address after growth
//! assert_eq!(ref0_addr, ref0_addr_after_growth); // the pushed elements are pinned
//!
//! // so it is safe to read from this memory location,
//! // which will return the correct data
//! let value0 = *ref0;
//! assert_eq!(value0, 0);
//! ```
//!
//! ## Safety-2
//!
//! On the other hand, the following operations would change the memory locations
//! of elements of the vector:
//!
//! * `insert`ing an element to an arbitrary location of the vector,
//! * `pop`ping or `remove`ing from the vector, or
//! * `clear`ing the vector.
//!
//! Therefore, similar to `Vec`, these operations require a mutable reference of `ImpVec`.
//! Thanks to the ownership rules, all references are dropped before using these operations.
//!
//! For instance, the following code safely will not compile.
//!
//! ```rust
//! use orx_imp_vec::ImpVec;
//!
//! let mut vec = ImpVec::default(); // mut required for the insert call
//!
//! // push the first item and hold a reference to it
//! let ref0 = vec.push_get_ref(0);
//!
//! // this is okay
//! vec.push(1);
//!
//! // this operation invalidates `ref0` which is now the address of value 42.
//! vec.insert(0, 42);
//! assert_eq!(vec, &[42, 0, 1]);
//!
//! // therefore, this line will lead to a compiler error!!
//! // let value0 = *ref0;
//! ```
//!
//! ## Practicality
//!
//! Being able to safely push to a collection with an immutable reference turns out to be very useful.
//!
//! You may see below how `ImpVec` helps to easily represent some tricky data structures.
//!
//! ### An alternative cons list
//!
//! Recall the classical [cons list example](https://doc.rust-lang.org/book/ch15-01-box.html).
//! Here is the code from the book which would not compile and used to discuss challenges and introduce smart pointers.
//!
//! ```ignore
//! enum List {
//!     Cons(i32, List),
//!     Nil,
//! }
//! fn main() {
//!     let list = Cons(1, Cons(2, Cons(3, Nil)));
//! }
//! ```
//!
//!
//! Below is a convenient cons list implementation using `ImpVec` as a storage:
//!
//! * to which we can immutably push new lists,
//! * while simultaneously holding onto and using references to already created lists.
//!
//! ```rust
//! use orx_imp_vec::ImpVec;
//!
//! enum List<'a, T> {
//!     Cons(T, &'a List<'a, T>),
//!     Nil,
//! }
//! let storage = ImpVec::default();
//! let r3 = storage.push_get_ref(List::Cons(3, &List::Nil));   // Cons(3) -> Nil
//! let r2 = storage.push_get_ref(List::Cons(2, r3));           // Cons(2) -> Cons(3)
//! let r1 = storage.push_get_ref(List::Cons(2, r2));           // Cons(2) -> Cons(1)
//! ```
//!
//! Alternatively, the `ImpVec` can be used only internally
//! leading to a cons list implementation with a nice api to build the list.
//!
//! The storage will keep growing seamlessly while making sure that
//! all references are **thin** and **valid**.
//!
//! ```rust
//! use orx_imp_vec::ImpVec;
//!
//! enum List<'a, T> {
//!     Cons(T, &'a List<'a, T>),
//!     Nil(ImpVec<List<'a, T>>),
//! }
//! impl<'a, T> List<'a, T> {
//!     fn storage(&self) -> &ImpVec<List<'a, T>> {
//!         match self {
//!             List::Cons(_, list) => list.storage(),
//!             List::Nil(storage) => storage,
//!         }
//!     }
//!     pub fn nil() -> Self {
//!         Self::Nil(ImpVec::default())
//!     }
//!     pub fn connect_from(&'a self, value: T) -> &Self {
//!         let new_list = Self::Cons(value, self);
//!         self.storage().push_get_ref(new_list)
//!     }
//! }
//!
//! let nil = List::nil();          // sentinel holds the storage
//! let r3 = nil.connect_from(3);   // Cons(3) -> Nil
//! let r2 = r3.connect_from(2);    // Cons(2) -> Cons(3)
//! let r1 = r2.connect_from(1);    // Cons(2) -> Cons(1)
//! ```
//!
//! ### Directed Acyclic Graph
//!
//! The cons list example reveals a pattern;
//! `ImpVec` can safely store and allow references when the structure is
//! built backwards starting from a sentinel node.
//!
//! Direct acyclic graphs (DAG) or trees are examples for such cases.
//! In the following, we define the Braess network as an example DAG, having edges:
//!
//! * A -> B
//! * A -> C
//! * B -> D
//! * C -> D
//! * B -> C (the link causing the paradox!)
//!
//! Such a graph could very simply constructed with an `ImpVec` where the nodes
//! are connected via regular references.
//!
//! ```rust
//! use orx_imp_vec::ImpVec;
//! use std::fmt::{Debug, Display};
//!
//! #[derive(PartialEq, Eq, Debug)]
//! struct Node<'a, T> {
//!     id: T,
//!     target_nodes: Vec<&'a Node<'a, T>>,
//! }
//! impl<'a, T: Debug + Display> Display for Node<'a, T> {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(
//!             f,
//!             "node: {}\t\tout-degree={}\tconnected-to={:?}",
//!             self.id,
//!             self.target_nodes.len(),
//!             self.target_nodes.iter().map(|n| &n.id).collect::<Vec<_>>()
//!         )
//!     }
//! }
//! #[derive(Default)]
//! struct Graph<'a, T>(ImpVec<Node<'a, T>>);
//! impl<'a, T> Graph<'a, T> {
//!     fn add_node(&self, id: T, target_nodes: Vec<&'a Node<'a, T>>) -> &Node<'a, T> {
//!         let node = Node { id, target_nodes };
//!         self.0.push_get_ref(node)
//!     }
//! }
//!
//! let graph = Graph::default();
//! let d = graph.add_node("D".to_string(), vec![]);
//! let c = graph.add_node("C".to_string(), vec![d]);
//! let b = graph.add_node("B".to_string(), vec![c, d]);
//! let a = graph.add_node("A".to_string(), vec![b, c]);
//!
//! for node in graph.0.into_iter() {
//! println!("{}", node);
//! }
//!
//! assert_eq!(2, a.target_nodes.len());
//! assert_eq!(vec![b, c], a.target_nodes);
//! assert_eq!(vec![c, d], a.target_nodes[0].target_nodes);
//! assert_eq!(vec![d], a.target_nodes[0].target_nodes[0].target_nodes);
//! assert!(a.target_nodes[0].target_nodes[0].target_nodes[0]
//!     .target_nodes
//!     .is_empty());
//! ```

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
