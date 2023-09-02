//! # orx-imp-vec
//!
//! An `ImpVec` wraps a vector implementing [`PinnedVec`](https://crates.io/crates/orx-pinned-vec), and hence, inherits the following feature:
//!
//! * the data stays **pinned** in place; i.e., memory location of an item added to the vector will never change unless the vector is dropped or cleared.
//!
//! Two main `PinnedVec` implementations which can be converted into an `ImpVec` are:
//!
//! * [`SplitVec`](https://crates.io/crates/orx-split-vec) which allows for flexible strategies to explicitly define how the vector should grow, and
//! * [`FixedVec`](https://crates.io/crates/orx-fixed-vec) with a strict predetermined capacity while providing the speed of a standard vector.
//!
//! Using the guarantees of `PinnedVec`, `ImpVec` provides additional abilities to push to or extend the vector with an immutable reference and provide a safe and convenient api to build vectors with self-referencing elements.
//!
//! Therefore, it is called the `ImpVec` 👿 standing for 'immutable push vector'.
//!
//! ## A. The goal
//!
//! Four relevant types work together towards a common goal as follows:
//!
//! * trait `PinnedVec` defines the safety guarantees for keeping the memory locations of already pushed elements;
//!     * struct `FixedVec` implements `PinnedVec` with a pre-determined fixed capacity while providing standard vector's complexity and performance;
//!     * struct `SplitVec` implements `PinnedVec` allowing for a dynamic capacity with an additional level of abstraction;
//! * struct `ImpVec` wraps any `PinnedVec` implementations and provides the safe api to allow for building vectors where elements may hold references to each other.
//!
//! Therefore, the main goal is to make it convenient and safe to build tricky data structures, child structures of which holds references to each other.
//! This is a common and a very useful pattern to represent structures such as trees, graphs or linked lists.
//!
//! Being able to represent such dependencies within a vector has the following advantages over alternative representations with smart pointers:
//!
//! * *better cache locality* →  child structures will be close to each other; therefore, following a path of relations through these references will visit elements of the same vector, rather than children allocated at arbitrary memory locations;
//! * *thin references* →  rather than wide pointers, the relations among elements of the vector are defined by plain references which also helps in avoiding heap allocations.
//!
//! ## B. Safety
//!
//! ### B.1. Safety: immutable push
//!
//! Pushing to a vector with an immutable reference sounds unsafe; however, `ImpVec` provides the safety guarantees.
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
//! However, `ImpVec` uses the `PinnedVec` as its underlying data which guarantees that the memory location of an item added to the vector will never change unless the vector is dropped or cleared.
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
//! ### B.2. Safety: reference breaking mutations
//!
//! On the other hand, the following operations would change the memory locations of elements of the vector:
//!
//! * `insert`ing an element to an arbitrary location of the vector,
//! * `pop`ping or `remove`ing from the vector,
//! * `swap`ping elements, or
//! * `truncate`-ing the vector.
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
//! ### B.3. Safety: reference breaking mutations for self referencing vectors
//!
//! On the other hand, when the element type is not a `NotSelfRefVecItem`, the above-mentioned mutations become much more significant.
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
//! Note that `Person` type is a self referencing vector item; and hence, is not a `NotSelfRefVecItem`.
//!
//! In the built `people` vector, jane helps john; which is represented in memory as `people[1]` helps `people[0]`.
//!
//! Now assume that we call `people.insert(0, mary)`. After this operation, the vector would be `[mary, john, jane]` breaking the relation between john and jane:
//!
//! * `people[1]` helps `people[0]` would now correspond to john helps mary,which is incorrect. In addition, `remove` and `pop` operations could further lead to undefined behavior.
//!
//! For this reason, these methods are not available when the element type is not `NotSelfRefVecItem`. Instead, there exist **unsafe** counterparts such as `unsafe_insert`.
//!
//! For similar reasons, `clone` is only available when the element type is `NotSelfRefVecItem`.
//!
//! ## C. Practicality
//!
//! ### C.1. Self referencing vectors (acyclic)
//!
//! Being able to safely push to a collection with an immutable reference turns out to be a convenient tool for building relationships among children of a parent structure.
//!
//! You may see below how `ImpVec` helps to easily represent some tricky data structures.
//!
//! #### C.1.a. An alternative cons list
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
//! use orx_imp_vec::prelude::*;
//!
//! #[derive(Debug)]
//! enum List<'a, T> {
//!     Cons(T, &'a List<'a, T>),
//!     Nil,
//! }
//! impl<'a, T: PartialEq> PartialEq for List<'a, T> {
//!     // compare references
//!     fn eq(&self, other: &Self) -> bool {
//!         let ptr_eq =
//! |l1, r1| std::ptr::eq(l1 as *const &'a List<'a, T>, r1 as *const &'a List<'a, T>);
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
//! Alternatively, the `ImpVec` can be used only internally leading to a cons list implementation with a nice api to build the list. The storage will keep growing seamlessly while making sure that all references are **thin** and **valid**.
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
//! #### C.1.b. Directed Acyclic Graph
//!
//! The cons list example reveals a pattern; `ImpVec` can safely store and allow references when the structure is built backwards starting from a sentinel node.
//!
//! Direct acyclic graphs (DAG) or trees are examples for such cases. In the following, we define the Braess network as an example DAG, having edges:
//!
//! * A -> B
//! * A -> C
//! * B -> D
//! * C -> D
//! * B -> C (the link causing the paradox!)
//!
//! Such a graph could be constructed very conveniently with an `ImpVec` where the nodes are connected via regular references.
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
//! ### C.2. Self referencing vectors (any!)
//!
//! As it has become apparent from the previous example, self referencing vectors can easily and conveniently be represented and built using an `ImpVec` provided that the references are acyclic. Although useful, this is limited.
//!
//! `ImpVec` provides further abilities to build cyclic references as well which requires only slightly more care. **orx-pinned-vec** crate defines a general purpose `SelfRefVecItem` trait which is able to define all practical relations among elements of the vector. All methods have default do-nothing implementations; therefore, only the relevant methods need to be implemented. `ImpVec` provides corresponding methods for conveniently and safely managing the relations among elements.
//!
//! #### C.2.a. Cyclic reference example
//!
//! Consider for instance a circle of people holding hands. We can define the complete circle by defining the person to the right of each person. Say our circle starts with: `a -> b -> c -> d -> a -> ...`. Note that we have a cyclic relation and we cannot build this only with the `push_get_ref` method. Further assume that people are switching places and we want to be able to update the relations. For the example, there will be a single such move where `b` and `c` will switch places leading to the new circle `a -> c -> b -> d -> a -> ...`.
//!
//! In this case, we only need to implement `next` (use next for right-to) and `set_next` methods of `SelfRefVecItem` trait; and this allows us to utilize `set_next` method of `ImpVec` to define and update relationships among people regardless of the relations being cyclic or acyclic.
//!
//! ```rust
//! use orx_imp_vec::prelude::*;
//!
//! struct Person<'a> {
//!     name: String,
//!     person_on_right: Option<&'a Person<'a>>,
//! }
//! impl<'a> Person<'a> {
//!     fn person_on_right_name(&self) -> Option<&'a str> {
//!         self.person_on_right.map(|p| p.name.as_str())
//!     }
//! }
//! impl<'a> SelfRefVecItem<'a> for Person<'a> {
//!     fn next(&self) -> Option<&'a Self> {
//!         self.person_on_right
//!     }
//!     fn set_next(&mut self, next: Option<&'a Self>) {
//!         self.person_on_right = next;
//!     }
//! }
//!
//! let mut people: ImpVec<_, _> = SplitVec::with_doubling_growth(4).into();
//!
//! // just push the people without the relationship
//! let names = &["a", "b", "c", "d"];
//! for name in names {
//!     people.push(Person {
//!         name: name.to_string(),
//!         person_on_right: None,
//!     });
//! }
//!
//! // define the circle: a -> b -> c -> d -> a -> ...
//! for i in 1..people.len() {
//!     people.set_next(i - 1, Some(i));
//! }
//! people.set_next(people.len() - 1, Some(0));
//!
//! assert_eq!(Some("b"), people[0].person_on_right_name()); // a -> b
//! assert_eq!(Some("c"), people[1].person_on_right_name()); // b -> c
//! assert_eq!(Some("d"), people[2].person_on_right_name()); // c -> d
//! assert_eq!(Some("a"), people[3].person_on_right_name()); // d -> a
//!
//! // now let b & c switch without any data copies
//! people.set_next(0, Some(2)); // a -> c
//! people.set_next(2, Some(1)); // c -> b
//! people.set_next(1, Some(3)); // b -> d
//!
//! assert_eq!(Some("c"), people[0].person_on_right_name()); // a -> c
//! assert_eq!(Some("d"), people[1].person_on_right_name()); // b -> d
//! assert_eq!(Some("b"), people[2].person_on_right_name()); // c -> b
//! assert_eq!(Some("a"), people[3].person_on_right_name()); // d -> a
//!
//! ```
//!
//! #### C.2.b. Crates utlizing ImpVec
//!
//! * [LinkedList](https://crates.io/crates/orx-linked-list): with the help of `ImpVec`, linked list is defined by only two `unsafe` method calls which are isolated only relevant to improve memory utilization.

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
mod self_ref;
mod test;

pub use common_traits::iter::iterator::{ImpVecIter, ImpVecIterMut};
pub use imp_vec::ImpVec;
