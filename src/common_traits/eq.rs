use crate::imp_vec::ImpVec;
use alloc::vec::Vec;
use orx_fixed_vec::FixedVec;
use orx_pinned_vec::PinnedVec;
use orx_split_vec::{Growth, SplitVec};

// imp

impl<T: PartialEq, P1: PinnedVec<T>, P2: PinnedVec<T>> PartialEq<ImpVec<T, P2>> for ImpVec<T, P1> {
    fn eq(&self, other: &ImpVec<T, P2>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

// split

impl<T: PartialEq, P: PinnedVec<T>, G: Growth> PartialEq<ImpVec<T, P>> for SplitVec<T, G> {
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: PartialEq, P: PinnedVec<T>, G: Growth> PartialEq<SplitVec<T, G>> for ImpVec<T, P> {
    fn eq(&self, other: &SplitVec<T, G>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

// fixed

impl<T: PartialEq, P: PinnedVec<T>> PartialEq<ImpVec<T, P>> for FixedVec<T> {
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: PartialEq, P: PinnedVec<T>> PartialEq<FixedVec<T>> for ImpVec<T, P> {
    fn eq(&self, other: &FixedVec<T>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

// vec

impl<T: PartialEq, P: PinnedVec<T>> PartialEq<ImpVec<T, P>> for Vec<T> {
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: PartialEq, P: PinnedVec<T>> PartialEq<Vec<T>> for ImpVec<T, P> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

// slice

impl<T: PartialEq, P: PinnedVec<T>> PartialEq<ImpVec<T, P>> for [T] {
    fn eq(&self, other: &ImpVec<T, P>) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: PartialEq, P: PinnedVec<T>> PartialEq<[T]> for ImpVec<T, P> {
    fn eq(&self, other: &[T]) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}
