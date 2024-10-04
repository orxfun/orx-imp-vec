use std::fmt::{Display, Formatter, Result};
use std::{cell::UnsafeCell, ops::Index};

struct Vector<'a> {
    symbol: String,
    created_vars: UnsafeCell<Vec<Var<'a>>>,
}

impl<'a> Vector<'a> {
    fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.into(),
            created_vars: Default::default(),
        }
    }
}

impl<'a> Index<usize> for &'a Vector<'a> {
    type Output = Var<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        let var = Var {
            index,
            vector: self,
        };

        let cache = unsafe { &mut *self.created_vars.get() };
        cache.push(var);
        &cache[cache.len() - 1]
    }
}

#[derive(Clone, Copy)]
struct Var<'a> {
    vector: &'a Vector<'a>,
    index: usize,
}

impl<'a> Display for Var<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}[{}]", &self.vector.symbol, self.index)
    }
}

fn main() {
    let x = &Vector::new("x");

    // good

    let x0: Var = x[0];
    assert_eq!(x0.to_string(), "x[0]");

    // also good

    let vars1: Vec<Var> = (0..1000).map(|i| x[i]).collect();

    for (i, x) in vars1.iter().enumerate() {
        assert_eq!(x.to_string(), format!("x[{}]", i));
    }

    // ¯\_(ツ)_/¯ UNDEFINED BEHAVIOR !!

    let vars2: Vec<&Var> = (0..1000).map(|i| &x[i]).collect();

    for (i, x) in vars2.iter().enumerate() {
        assert_eq!(x.to_string(), format!("x[{}]", i));
    }
}
