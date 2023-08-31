//! An `ImpVec` wraps a vector implementing [`PinnedVec`](https://crates.io/crates/orx-pinned-vec),
//! and hence, inherits the following feature:
//!
//! * the data stays **pinned** in place; i.e., memory location of an item added to the vector
//! will never change unless the vector is dropped or cleared.
//!
//! Two main `PinnedVec` implementations which can be converted into an `ImpVec` are:
//!
//! * [`SplitVec`](https://crates.io/crates/orx-split-vec) which allows for
//! flexible strategies to explicitly define how the vector should grow, and
//! * [`FixedVec`](https://crates.io/crates/orx-fixed-vec) with a strict predetermined capacity
//! while providing the speed of a standard vector.
//!
//! Making use of the interior mutability and pinned elements property of the underlying pinned vector,
//! an `ImpVec` allows to safely push to or extend the vector with an **immutable reference**;
//! hence, it gets the name `ImpVec` standing for 'immutable push vector'.
//! It also hints for the little evil behavior 👿 it has.
//!
//! ## Main goal
//!
//! The main purpose of `PinnedVec` implementations is to represent complex data structures,
//! child structures of which often holds references to each other.
//! This is a common and useful property to represent structures such as trees and graphs.
//! Pinned vector represents such structures while keeping the child structures in a vector-like
//! layout.
//! Compared to alternative representations with smart pointers, this representation provides
//! the following advantages:
//!
//! * holding children close to each other provides better cache locality,
//! * reduces heap allocations and utilizes **thin** references rather than wide pointers,
//! * while still guaranteeing that the references will remain valid.
//!
//! The `ImpVec`, on the other hand, wraps the `PinnedVec` and allows the vector to grow safely
//! with an immutable reference using interior mutability.
//! This enables building complex data structures represented as vectors with self referencing
//! elements.
//!
//! Eventually, the `ImpVec` can be converted back to its underlying `PinnedVec`
//! to drop interior mutability and reduce the level of abstraction.
//!
//! ## Safety: immutable push
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
//! Why does `push` invalidate the reference to the first element?
//! * the vector has a capacity of 2; and hence, the push leads to an expansion of the vector's capacity;
//! * it is possible that the underlying data will be copied to another place in memory;
//! * in this case `ref0` will be an invalid reference and dereferencing it would lead to an undefined behavior (UB).
//!
//! However, `ImpVec` uses the `PinnedVec` as its underlying data
//! which guarantees that the memory location of an item added to the vector will never change
//! unless the vector is dropped or cleared.
//!
//! Therefore, the following `ImpVec` version compiles and preserves the validity of the references.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! let vec: ImpVec<_, _> = SplitVec::with_doubling_growth(2).into();
//! vec.push(0);
//! vec.push(1);
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
//! ## Safety: reference breaking mutations
//!
//! On the other hand, the following operations would change the memory locations
//! of elements of the vector:
//!
//! * `insert`ing an element to an arbitrary location of the vector,
//! * `pop`ping or `remove`ing from the vector.
//!
//! Therefore, similar to `Vec`, these operations require a mutable reference of `ImpVec`.
//! Thanks to the ownership rules, all references are dropped before using these operations.
//!
//! For instance, the following code safely will not compile.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! let mut vec: ImpVec<_, _> = SplitVec::with_linear_growth(4).into(); // mut required for the insert call
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
//! ## Safety: reference breaking mutations for self referencing vectors
//!
//! On the other hand, when the element type is not a `NotSelfRefVecItem`,
//! the above-mentioned mutations become more dangerous.
//!
//! Consider the following example.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! struct Person<'a> {
//!     name: String,
//!     helps: Option<&'a Person<'a>>,
//! }
//!
//! let mut people: ImpVec<_, _> = SplitVec::with_linear_growth(4).into();
//!
//! let john = people.push_get_ref(Person {
//!     name: String::from("john"),
//!     helps: None,
//! });
//! people.push(Person {
//!     name: String::from("jane"),
//!     helps: Some(john),
//! });
//!
//! assert_eq!(None, people[0].helps.map(|x| x.name.as_str()));
//! assert_eq!(Some("john"), people[1].helps.map(|x| x.name.as_str()));
//! ```
//!
//! Note that `Person` type is a self referencing vector item;
//! and hence, is not a `NotSelfRefVecItem`.
//!
//! In the built `people` vector, jane helps john;
//! which is represented as `people[1]` helps `people[0]`.
//!
//! Now assume that we call `people.insert(0, mary)`.
//! After this operation, the vector would be `[mary, john, jane]` breaking the relation between john and jane:
//!
//! * `people[1]` helps `people[0]` would now correspond to john helps mary, which is incorrect.
//!
//! In addition to incorrectness, `remove` and `pop` operations could further lead to undefined behavior.
//!
//! For this particular reason,
//! these methods are not available when the element type is not `NotSelfRefVecItem`.
//! Instead, there exist **unsafe** counterparts such as `unsafe_insert`.
//!
//! For similar reasons, `clone` is only available when the element type is `NotSelfRefVecItem`.
//!
//!
//! ## Practicality - Self referencing vectors
//!
//! Being able to safely push to a collection with an immutable reference turns out to be very useful.
//! Self-referencing vectors can be conveniently built;
//! in particular, vectors where elements hold a reference to other elements of the vector.
//!
//! You may see below how `ImpVec` helps to easily represent some tricky data structures.
//!
//! ### An alternative cons list
//!
//! Recall the classical [cons list example](https://doc.rust-lang.org/book/ch15-01-box.html).
//! Here is the code from the book which would not compile and used to discuss challenges and introduce smart pointers.
//!
//! ```rust ignore
//! enum List {
//!     Cons(i32, List),
//!     Nil,
//! }
//!
//! let list = Cons(1, Cons(2, Cons(3, Nil)));
//! ```
//!
//!
//! Below is a convenient cons list implementation using `ImpVec` as a storage:
//!
//! * to which we can immutably push new lists,
//! * while simultaneously holding onto and using references to already created lists.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! #[derive(Debug)]
//! enum List<'a, T> {
//!     Cons(T, &'a List<'a, T>),
//!     Nil,
//! }
//! impl<'a, T: PartialEq> PartialEq for List<'a, T> {
//!     fn eq(&self, other: &Self) -> bool { // compare references
//!         let ptr_eq =
//!             |l1, r1| std::ptr::eq(l1 as *const &'a List<'a, T>, r1 as *const &'a List<'a, T>);
//!         match (self, other) {
//!             (Self::Cons(l0, l1), Self::Cons(r0, r1)) => l0 == r0 && ptr_eq(l1, r1),
//!             _ => core::mem::discriminant(self) == core::mem::discriminant(other),
//!         }
//!     }
//! }
//! impl<'a, T> List<'a, T> {
//!     fn cons(&self) -> Option<&'a List<'a, T>> {
//!         match self {
//!             List::Nil => None,
//!             List::Cons(_, x) => Some(*x),
//!         }
//!     }
//! }
//!
//! let lists: ImpVec<_, _> = SplitVec::with_exponential_growth(10, 1.5).into();
//! let nil = lists.push_get_ref(List::Nil); // Nil
//! let r3 = lists.push_get_ref(List::Cons(3, nil)); // Cons(3) -> Nil
//! let r2 = lists.push_get_ref(List::Cons(42, r3)); // Cons(42) -> Cons(3)
//! let r1 = lists.push_get_ref(List::Cons(42, r2)); // Cons(42) -> Cons(42)
//!
//! assert_eq!(r1.cons(), Some(r2));
//! assert_eq!(r2.cons(), Some(r3));
//! assert_eq!(r3.cons(), Some(nil));
//! assert_eq!(nil.cons(), None);
//!
//! // use index in the outer collection
//! assert_eq!(r1, &lists[3]);
//!
//! // both are Cons variant with value 42; however, pointing to different list
//! assert_ne!(r2, r3);
//! ```
//!
//! Alternatively, the `ImpVec` can be used only internally
//! leading to a cons list implementation with a nice api to build the list.
//!
//! The storage will keep growing seamlessly while making sure that
//! all references are **thin** and **valid**.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//! type ImpVecLin<T> = ImpVec<T, SplitVec<T>>;
//!
//! enum List<'a, T> {
//!     Cons(T, &'a List<'a, T>),
//!     Nil(ImpVecLin<List<'a, T>>),
//! }
//! impl<'a, T> List<'a, T> {
//!     fn storage(&self) -> &ImpVecLin<List<'a, T>> {
//!         match self {
//!             List::Cons(_, list) => list.storage(),
//!             List::Nil(storage) => storage,
//!         }
//!     }
//!     pub fn nil() -> Self {
//!         Self::Nil(ImpVecLin::default())
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
//! Such a graph could be constructed very conveniently with an `ImpVec` where the nodes
//! are connected via regular references.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//! use std::fmt::Debug;
//!
//! #[derive(PartialEq, Eq)]
//! struct Node<'a, T> {
//!     id: T,
//!     target_nodes: Vec<&'a Node<'a, T>>,
//! }
//! impl<'a, T: Debug> Debug for Node<'a, T> {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(
//!             f,
//!             "node({:?})\t\tout-degree={}\t\tconnected-to={:?}",
//!             self.id,
//!             self.target_nodes.len(),
//!             self.target_nodes.iter().map(|n| &n.id).collect::<Vec<_>>()
//!         )
//!     }
//! }
//!
//! #[derive(Default)]
//! struct Graph<'a, T>(ImpVec<Node<'a, T>, SplitVec<Node<'a, T>, DoublingGrowth>>);
//!
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
//!     println!("{:?}", node);
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
//!
//! ### Practicality (unsafe) - Cyclic References
//!
//! As it has become apparent from the previous example,
//! self referencing vectors can easily and conveniently be represented and built using an `ImpVec`
//! provided that the references are acyclic.
//!
//! In addition, using the unsafe `get_mut` method,
//! cyclic self referencing vectors can be represented.
//! Consider for instance, the following example where
//! the vector contains two points pointing to each other.
//! This cyclic relation can be represented with the unsafe call to the `get_mut` method.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! struct Point<'a, T> {
//!     data: T,
//!     next: Option<&'a Point<'a, T>>,
//! }
//!
//! // cyclic reference of two points: Point(even) <--> Point(odd)
//! let even_odd: ImpVec<_, _> = FixedVec::new(2).into();
//!
//! let even = even_odd.push_get_ref(Point {
//!     data: 'e',
//!     next: None, /*none for now*/
//! });
//! let odd = even_odd.push_get_ref(Point {
//!     data: 'o',
//!     next: Some(even),
//! });
//!
//! // close the circle
//! unsafe { even_odd.get_mut(0) }.unwrap().next = Some(odd);
//!
//! let mut curr = even;
//! for i in 0..42 {
//!     if i % 2 == 0 {
//!         assert_eq!('e', curr.data);
//!     } else {
//!         assert_eq!('o', curr.data);
//!     }
//!     curr = curr.next.unwrap();
//! }
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

mod common_traits;
mod get_mut;
mod imp_vec;
mod index;
/// Common traits, structs, enums.
pub mod prelude;
mod push;
mod test;

pub use common_traits::iter::iterator::{ImpVecIter, ImpVecIterMut};
pub use imp_vec::ImpVec;
