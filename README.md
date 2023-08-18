# orx-imp-vec

An `ImpVec` uses [`SplitVec`](https://crates.io/crates/orx-split-vec) as the underlying data model and inherits its features.

Additionally, it allows to push to or extend the vector with an immutable reference; hence, the name:

* imp-vec stands for 'immutable push vector',
* and also hints a little evil behavior it has.


## Safety-1

Pushing to a vector with an immutable reference sounds unsafe;
however, `ImpVec` provides the safety guarantees.

Consider the following example using `std::vec::Vec` which does not compile:

```rust
let mut vec = Vec::with_capacity(2);
vec.extend_from_slice(&[0, 1]);

let ref0 = &vec[0];
vec.push(2);
let value0 = *ref0;
```

Why does `push` invalidate the reference to the first element which is already pushed to the vector:
* the vector has a capacity of 2; and hence, the push will lead to an expansion of the vector's capacity;
* it is possible that the underlying data will be copied to another place in memory;
* in this case `ref0` will be an invalid reference and dereferencing it would lead to UB.

However, `ImpVec` uses the `SplitVec` as its underlying data model
which guarantees that the memory location of an item added to the split vector will never change
unless it is removed from the vector or the vector is dropped.

Therefore, the  following `ImpVec` version compiles and maintains the validity of the references.

```rust
use orx_split_vec::FragmentGrowth;
use orx_imp_vec::ImpVec;

let vec = ImpVec::with_growth(FragmentGrowth::constant(2));
vec.extend_from_slice(&[0, 1]);

let ref0 = &vec[0];
let ref0_addr = ref0 as *const i32; // address before growth

vec.push(2); // capacity is increased here

let ref0_addr_after_growth = &vec[0] as *const i32; // address after growth
assert_eq!(ref0_addr, ref0_addr_after_growth); // the pushed elements are pinned

// so it is safe to read from this memory location,
// which will return the correct data
let value0 = *ref0;
assert_eq!(value0, 0);
```

## Safety-2

On the other hand, the following operations would change the memory locations
of elements of the vector:

* `insert`ing an element to an arbitrary location of the vector,
* `pop`ping or `remove`ing from the vector, or
* `clear`ing the vector.

Therefore, similar to `Vec`, these operations require a mutable reference of `ImpVec`.
Thanks to the ownership rules, all references are dropped before using these operations.

For instance, the following code safely will not compile.

```rust
use orx_imp_vec::ImpVec;

let mut vec = ImpVec::default();

// push the first item and hold a reference to it
let ref0 = vec.push_get_ref(0);

// this is okay
vec.push(1);

// this operation invalidates `ref0` which is now the address of value 42.
vec.insert(0, 42);
assert_eq!(vec, &[42, 0, 1]);

// therefore, this line will lead to a compiler error.
let value0 = *ref0;
```

## Practicality

Being able to safely push to a collection with an immutable reference turns out to be very useful.

You may see below how `ImpVec` helps to easily represent some tricky data structures.

### An alternative cons list

Recall the classical [cons list example](https://doc.rust-lang.org/book/ch15-01-box.html).
Here is the code from the book which would not compile and used to discuss challenges and introduce smart pointers.

```rust
enum List {
    Cons(i32, List),
    Nil,
}
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```


Below is a convenient cons list implementation using `ImpVec` as a storage:

* to which we can immutably push new lists,
* while simultaneously holding onto and using references to already created lists.

```rust
enum List<'a, T> {
    Cons(T, &'a List<'a, T>),
    Nil,
}
fn main() {
    let storage = ImpVec::default();
    let r3 = storage.push_get_ref(List::Cons(3, &List::Nil));   // Cons(3) -> Nil
    let r2 = storage.push_get_ref(List::Cons(2, r3));           // Cons(2) -> Cons(3)
    let r1 = storage.push_get_ref(List::Cons(2, r2));           // Cons(2) -> Cons(1)
}
```

Alternatively, the `ImpVec` can be used only internally
leading to a cons list implementation with a nice api to build the list.

```rust
enum List<'a, T> {
    Cons(T, &'a List<'a, T>),
    Nil(Rc<ImpVec<T>>),
}
impl<'a, T> List<'a, T> {
    fn storage(&self) -> Rc<ImpVec<T>> {
        match self {
            List::Cons(_, list) => list.storage(),
            List::Nil(s) => s.clone(),
        }
    }
    pub fn nil() -> Self {
        Self::Nil(Rc::new(ImpVec::default()))
    }
    pub fn connect_from(&'a self, value: T) -> Self {
        Self::Cons(value, self)
    }
}
fn main2() {
    let nil = List::nil();          // sentinel holds the storage
    let r3 = nil.connect_from(3);   // Cons(3) -> Nil
    let r2 = r3.connect_from(2);    // Cons(2) -> Cons(3)
    let r1 = r2.connect_from(1);    // Cons(2) -> Cons(1)
}
```
