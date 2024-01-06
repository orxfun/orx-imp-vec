use crate::ImpVec;
use orx_pinned_vec::{
    self_referential_elements::{SelfRefNext, SelfRefPrev},
    PinnedVec,
};

impl<'a, T, P> ImpVec<T, P>
where
    P: PinnedVec<T> + 'a,
    T: SelfRefNext<'a> + 'a,
{
    /// Using interior mutability peforms the following:
    ///
    /// * `element.set_next(next)`
    ///
    /// so that `element` will point to the `next` element after the operation.
    ///
    /// # Panics
    ///
    /// Panics:
    ///
    /// * if `element` does not belong to this vector, or
    /// * if `next` is of Some variant and underlying reference does not belong to this vector.
    ///
    /// # Safety
    ///
    /// Due to the guards defined in the Panics section, `element` and `next` (if some) belong to the same vector living together and sharing the same lifetime.
    ///
    /// # Example
    ///
    /// This is one of the specialized methods for building self-referential-collections when the elements implement `orx_pinned_vec::SelfRefNext`.
    ///
    /// It is not trivial to provide a partial and trivial example.
    /// However, the usefulness of this method can be demonstrated by its usage within the [`orx_linked_list::LinkedList`](https://crates.io/crates/orx-linked-list) implementation.
    ///
    /// The crate defines a double-linked-list `Node` with optional `next` and `prev` references.
    /// `Node` implements `orx_pinned_vec::SelfRefNext` and `orx_pinned_vec::SelfRefPrev`; therefore, we can use `ImpVec::set_next` method.
    /// The following code block is the `push_back` method implementation of the `LinkedList`:
    ///
    /// ```rust ignore
    /// pub fn push_back(&mut self, value: T) {
    ///     match self.back_node() {
    ///         None => self.push_first_node(value),
    ///         Some(old_back) => {
    ///             let node = Node::active(value, Some(old_back), None);
    ///             let back = unsafe { self.vec.push_get_ref(node) };
    ///             self.vec.set_next(old_back, Some(back));
    ///             self.slice = LinkedListSlice::new(self.len() + 1, self.front_node(), Some(back));
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// In the trivial case when there is no `back_node` (the list is empty), we simply push the `value` as the first node.
    ///
    /// Otherwise:
    /// * We get a reference to the back node, we'll now call it `old_back` since it will no longer be the back of the list.
    /// * We create a new `node` for the new `value`:
    ///   * previous node of `node` is set to `Some(old_back)`,
    ///   * next node of `node` is `None` since it is the new back of the list.
    /// * We use the `push_get_ref` method to push `node` to the storage vector and get a reference to it.
    ///   * SAFETY: we must make sure that the reference `back` does not outlive `self`; which is satisfied here.
    /// * We tell our `self.vec` which is an `ImpVec` to set the next of `old_back` to `Some(back)` to link the old back to the new back.
    /// * Then, we set that our list now is one element longer, with the old `self.front_node()` and new back node which is `Some(back)`.
    pub fn set_next(&mut self, element: &T, next: Option<&'a T>) {
        if let Some(next) = next {
            self.index_of(next).expect(INVALID_NEXT);
        }

        let node = self
            .index_of(element)
            .and_then(|idx| unsafe { self.get_mut(idx) })
            .expect(INVALID_ELEM);

        node.set_next(next)
    }
}

impl<'a, T, P> ImpVec<T, P>
where
    P: PinnedVec<T> + 'a,
    T: SelfRefPrev<'a> + 'a,
{
    /// Using interior mutability peforms the following:
    ///
    /// * `element.set_prev(prev)`
    ///
    /// so that `element` will point to the `prev` element after the operation.
    ///
    /// # Panics
    ///
    /// Panics:
    ///
    /// * if `element` does not belong to this vector, or
    /// * if `prev` is of Some variant and underlying reference does not belong to this vector.
    ///
    /// # Safety
    ///
    /// Due to the guards defined in the Panics section, `element` and `prev` (if some) belong to the same vector living together and sharing the same lifetime.
    ///
    /// # Example
    ///
    /// This is one of the specialized methods for building self-referential-collections when the elements implement `orx_pinned_vec::SelfRefNext`.
    ///
    /// It is not trivial to provide a partial and trivial example.
    /// However, the usefulness of this method can be demonstrated by its usage within the [`orx_linked_list::LinkedList`](https://crates.io/crates/orx-linked-list) implementation.
    ///
    /// The crate defines a double-linked-list `Node` with optional `next` and `prev` references.
    /// `Node` implements `orx_pinned_vec::SelfRefNext` and `orx_pinned_vec::SelfRefPrev`; therefore, we can use `ImpVec::set_next` method.
    /// The following code block is the `push_front` method implementation of the `LinkedList`:
    ///
    /// ```rust ignore
    /// pub fn push_front(&mut self, value: T) {
    ///     match self.front_node() {
    ///         None => self.push_first_node(value),
    ///         Some(old_front) => {
    ///             let node = Node::active(value, None, Some(old_front));
    ///             let front = unsafe { self.vec.push_get_ref(node) };
    ///             self.vec.set_prev(old_front, Some(front));
    ///             self.slice = LinkedListSlice::new(self.len() + 1, Some(front), self.back_node());
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// In the trivial case when there is no `front_node` (the list is empty), we simply push the `value` as the first node.
    ///
    /// Otherwise:
    /// * We get a reference to the front node, we'll now call it `old_front` since it will no longer be the front of the list.
    /// * We create a new `node` for the new `value`:
    ///   * next node of `node` is set to `Some(old_front)`,
    ///   * previous node of `node` is `None` since it is the new front of the list.
    /// * We use the `push_get_ref` method to push `node` to the storage vector and get a reference to it.
    ///   * SAFETY: we must make sure that the reference `front` does not outlive `self`, which is satisfied here.
    /// * We tell our `self.vec` which is an `ImpVec` to set the prev of `old_front` to `Some(front)` to link the old front node to the new front ndoe.
    /// * Then, we set that our list now is one element longer, with the old `self.back_node()` and new front node which is `Some(front)`.
    pub fn set_prev(&mut self, element: &T, prev: Option<&'a T>) {
        if let Some(prev) = prev {
            self.index_of(prev).expect(INVALID_PREV);
        }

        let node = self
            .index_of(element)
            .and_then(|idx| unsafe { self.get_mut(idx) })
            .expect(INVALID_ELEM);

        node.set_prev(prev)
    }
}

const INVALID_ELEM: &str = "element does not belong to this vector";
const INVALID_NEXT: &str = "next does not belong to this vector; ImpVec allows inter-element references only if both elements belong to the same ImpVec";
const INVALID_PREV: &str = "prev does not belong to this vector; ImpVec allows inter-element references only if both elements belong to the same ImpVec";

#[cfg(test)]
mod tests {
    use crate::{prelude::*, test_all_pinned_types};

    struct Node<'a> {
        data: char,
        prev: Option<&'a Self>,
        next: Option<&'a Self>,
    }

    impl<'a> SelfRefNext<'a> for Node<'a> {
        #[inline(always)]
        fn next(&self) -> Option<&'a Self> {
            self.next
        }
        #[inline(always)]
        fn set_next(&mut self, next: Option<&'a Self>) {
            self.next = next;
        }
    }

    impl<'a> SelfRefPrev<'a> for Node<'a> {
        #[inline(always)]
        fn prev(&self) -> Option<&'a Self> {
            self.prev
        }
        #[inline(always)]
        fn set_prev(&mut self, prev: Option<&'a Self>) {
            self.prev = prev;
        }
    }

    #[test]
    fn set_next_prev() {
        fn test<'a, P: PinnedVec<Node<'a>> + 'a>(pinned_vec: P) {
            let mut imp: ImpVec<_, _> = pinned_vec.into();

            let node = Node {
                data: 'a',
                prev: None,
                next: None,
            };
            let a = unsafe { imp.push_get_ref(node) };

            let node = Node {
                data: 'b',
                prev: None,
                next: None,
            };
            let b = unsafe { imp.push_get_ref(node) };

            for x in imp.iter() {
                assert!(x.next().is_none());
                assert!(x.prev().is_none());
            }

            imp.set_next(a, Some(b));
            imp.set_prev(b, Some(a));

            assert_eq!(a.next().map(|x| &x.data), Some(&'b'));
            assert_eq!(b.prev().map(|x| &x.data), Some(&'a'));

            imp.set_next(a, None);
            imp.set_prev(b, None);

            for x in imp.iter() {
                assert!(x.next().is_none());
                assert!(x.prev().is_none());
            }
        }

        test_all_pinned_types!(test);
    }

    #[test]
    #[should_panic]
    fn set_next_of_another_impvec() {
        let mut imp1: ImpVec<_> = Default::default();
        let mut imp2: ImpVec<_> = Default::default();

        let node = Node {
            data: 'a',
            prev: None,
            next: None,
        };
        let a = unsafe { imp1.push_get_ref(node) };

        let node = Node {
            data: 'b',
            prev: None,
            next: None,
        };
        let b = unsafe { imp2.push_get_ref(node) };

        imp2.set_next(a, Some(b));
    }

    #[test]
    #[should_panic]
    fn set_next_to_another_impvec() {
        let mut imp1: ImpVec<_> = Default::default();
        let mut imp2: ImpVec<_> = Default::default();

        let node = Node {
            data: 'a',
            prev: None,
            next: None,
        };
        let a = unsafe { imp1.push_get_ref(node) };

        let node = Node {
            data: 'b',
            prev: None,
            next: None,
        };
        let b = unsafe { imp2.push_get_ref(node) };

        imp1.set_next(a, Some(b));
    }

    #[test]
    #[should_panic]
    fn set_prev_to_another_impvec() {
        let mut imp1: ImpVec<_> = Default::default();
        let mut imp2: ImpVec<_> = Default::default();

        let node = Node {
            data: 'a',
            prev: None,
            next: None,
        };
        let a = unsafe { imp1.push_get_ref(node) };

        let node = Node {
            data: 'b',
            prev: None,
            next: None,
        };
        let b = unsafe { imp2.push_get_ref(node) };

        imp1.set_prev(a, Some(b));
    }

    #[test]
    #[should_panic]
    fn set_prev_of_another_impvec() {
        let mut imp1: ImpVec<_> = Default::default();
        let mut imp2: ImpVec<_> = Default::default();

        let node = Node {
            data: 'a',
            prev: None,
            next: None,
        };
        let a = unsafe { imp1.push_get_ref(node) };

        let node = Node {
            data: 'b',
            prev: None,
            next: None,
        };
        let b = unsafe { imp2.push_get_ref(node) };

        imp2.set_prev(a, Some(b));
    }
}
