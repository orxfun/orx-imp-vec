# orx-imp-vec

A data structure to enable conveniently building performant self referential collections such as trees, graphs and linked lists.

`ImpVec` 👿 is a wrapper around a [`PinnedVec`](https://crates.io/crates/orx-pinned-vec) which guarantees that memory location of its elements do not change unless elements are removed from the vector. In addition to dereferencing the `PinnedVec` methods, `ImpVec` adds the following methods which are specialized for defining and building **self referential collections** using plain `&` references:

```rust ignore
fn set_next(&mut self, element: &T, next: Option<&'a T>)
fn set_prev(&mut self, element: &T, prev: Option<&'a T>)
fn replace_at(&mut self, at: usize, new_value: T) -> Option<T>
unsafe fn push_get_ref(&self, value: T) -> &T
unsafe fn move_get_ref(&mut self, source_idx: usize, destination_idx: usize, fill_source_with: T) -> Option<(&T, &T)>
```

## A. Motivation

Self referential, often recursive, collections contain an important set of useful data structures such as trees, graphs and linked lists. However, building these structures with references `&` is not possible in safe rust.

`ImpVec` aims at allowing to safely build such structures without using pointers and limiting the unsafe code to the abovementioned well-documented methods which are specialized for building these collections.

*Although the practical usefulness of linked lists might not be clear, it sets a great example for the challenge of building self referential collections due to its simplicity. For this purpose, a linked list is used as the example to demonstrate the motivation; however, the approach can be generalized to other self referential collections.*

### Standard LinkedList

Alternatively, these collections can be built using reference counted smart pointers such as `std::rc::Rc` and independent heap allocations. However, independent heap allocations is a drawback as the elements do not live close to each other leading to poor cache locality. Further, reference counted pointers have a runtime overhead. `std::collections::LinkedList` implementation avoids reference counted pointers and uses `NonNull` instead, most likely to avoid this overhead. However, this leads to a risky and difficult implementation that feels more low level than it should. You may see the implementation [here](https://doc.rust-lang.org/src/alloc/collections/linked_list.rs.html). The `unsafe` keyword is used more than 60 times in this file. These are usually related to reading from and writing to memory through raw pointers.

***Motivation:*** We do not need to count references provided that all elements and inter-element references belong to the same owner or container. This is because all elements will be dropped at the same time together with their inter-element references when the container `ImpVec` is dropped.

***Motivation:*** We should be able to define these structures without directly accessing memory through raw pointers. This is unnecessarily powerful and risky. Instead, unsafe code must be limited to methods which are specialized for and only allow defining required connections of self referential collections.

### Using ImpVec: [orx_linked_list::LinkedList](https://crates.io/crates/orx-linked-list)

`orx_linked_list::LinkedList` on the other hand uses an `ImpVec` as the underlying storage and makes use of its specialized methods. This brings the following advantages:

* Allows for a higher level implementation without any use of raw pointers.
* Avoids smart pointers.
* Avoids almost completely accessing through integer indices.
* All nodes belong to the same `ImpVec` living close to each other. This allows for better cache locality.
* Full fetched doubly-linked-list implementation uses the `unsafe` keyword seven times, which are repeated uses of three methods:
  * `ImpVec::push_get_ref`
  * `ImpVec::move_get_ref`
  * `ImpVec::unsafe_truncate` (*a deref method from `PinnedVec`*)

Furthermore, this implementation is more performant than the standard library implementation.

## B. Example

```rust
use orx_imp_vec::prelude::*;

// from PinnedVec implementations
let imp: ImpVec<i32, _> = SplitVec::new().into();
let imp: ImpVec<i32, _> = SplitVec::with_doubling_growth().into();
let imp: ImpVec<i32, _> = SplitVec::with_linear_growth(10).into();
let imp: ImpVec<i32, _> = FixedVec::new(1024).into();

// or using default PinnedVec implementation
let mut imp: ImpVec<i32> = ImpVec::default();
let mut imp: ImpVec<i32> = ImpVec::new();

imp.push(42);
imp.extend_from_slice(&[1, 2, 3, 4]);
_ = imp.pop();
assert_eq!(imp.get(0), Some(&42));
assert_eq!(imp[0], 42);
assert_eq!(imp, &[42, 1, 2, 3]);

// into the underlying pinned vector
let pinned: SplitVec<_> = imp.into_pinned();
assert_eq!(pinned, &[42, 1, 2, 3]);
```

## C. Performance

### As a Vector

`ImpVec` wraps and dereferences an underlying `PinnedVec`. In other words, performance of regular vector operations such as random access by index or iteration over all elements is determined by that of the underlying pinned vector. Two efficient pinned vector implementations are [`orx_split_vec::SplitVec`](https://crates.io/crates/orx-split-vec) (default) and [`orx_fixed_vec::FixedVec`](https://crates.io/crates/orx-fixed-vec) have performance equal to or very close to that of `std::vec::Vec`.

However, it is still useful to visit all elements in any order. For such unordered iterations, the efficiency of standard vector operations becomes valuable.


### As a Self Rerefential Collection

However, `ImpVec` is useful in defining self referential structures which have their own special traversals following element-to-element references. Compared to counterpart implementations where elements are independently allocated, the `ImpVec` approach takes benefit of storing elements close to each other which leads to better cache locality.


## D. Collections built on `ImpVec`

* [orx-linked-list::LinkedList](https://crates.io/crates/orx-linked-list)

***why named `ImpVec`?***

*It took me a million iterations to figure out a nice way to move unsafety into a small set of methods to allow building self referential collections. In early attempts, `ImpVec` allowed for pushing to the vector with an immutable reference; and hence, it is named imp standing for 'immutable push'. This was sufficient to build acyclic collections such as trees; however, not sufficient for others such as graphs or doubly-linked-lists. In the current version, unsafety is limited to `push_get_ref` and `move_get_ref` methods which focus on lifetimes to allow the inter-element relations (so it was lifetimes, it's always lifetimes). Now, we do not need immutable push, and hence, `ImpVec` must mean something else.*


## License

This library is licensed under MIT license. See LICENSE for details.
