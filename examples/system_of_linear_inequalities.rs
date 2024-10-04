use orx_imp_vec::*;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Index, Mul},
};

/// # Scope
///
/// It is a bag of things,
/// analogous to variables, expressions, etc. defined
/// in the scope of a code block.
#[derive(Default)]
struct Scope<'a> {
    vectors: ImpVec<Vector<'a>>,
    exprs: ImpVec<Expr<'a>>,
    terms: ImpVec<Term<'a>>,
    vars: ImpVec<Var<'a>>,
}

impl<'a> Scope<'a> {
    fn same_scope_as(&self, other: &Self) -> bool {
        self as *const Self == other as *const Self
    }
}

impl<'a> Scope<'a> {
    fn new_var_vec(&'a self, symbol: &str) -> &'a Vector<'a> {
        self.vectors.imp_push_get_ref(Vector {
            scope: self,
            symbol: symbol.to_string(),
        })
    }
}

/// # VarVec
struct Vector<'a> {
    scope: &'a Scope<'a>,
    symbol: String,
}

impl<'a> Display for Vector<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.symbol)
    }
}

impl<'a> Index<usize> for &'a Vector<'a> {
    type Output = Var<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.scope.vars.imp_push_get_ref(Var {
            scope: self.scope,
            var_vec: self,
            index,
        })
    }
}

/// # Expr
struct Expr<'a> {
    scope: &'a Scope<'a>,
    terms: Vec<&'a Term<'a>>,
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut terms = self.terms.iter();
        if let Some(term) = terms.next() {
            write!(f, "{}", term)?;
            for term in terms {
                write!(f, " + {}", term)?;
            }
        }
        Ok(())
    }
}

impl<'a> Add<&'a Term<'a>> for &'a Expr<'a> {
    type Output = &'a Expr<'a>;

    fn add(self, rhs: &'a Term<'a>) -> Self::Output {
        assert!(self.scope.same_scope_as(rhs.scope));

        let mut terms = self.terms.clone();
        terms.push(rhs);
        self.scope.exprs.imp_push_get_ref(Expr {
            scope: self.scope,
            terms,
        })
    }
}

/// # Term
#[derive(Clone, Copy)]
struct Term<'a> {
    scope: &'a Scope<'a>,
    coef: i64,
    var: Var<'a>,
}

impl<'a> Display for Term<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}.{}", self.coef, self.var)
    }
}

impl<'a> Add<&'a Term<'a>> for &'a Term<'a> {
    type Output = &'a Expr<'a>;

    fn add(self, rhs: &'a Term<'a>) -> Self::Output {
        assert!(self.scope.same_scope_as(rhs.scope));

        self.scope.exprs.imp_push_get_ref(Expr {
            scope: self.scope,
            terms: vec![self, rhs],
        })
    }
}

/// # Var
#[derive(Clone, Copy)]
struct Var<'a> {
    scope: &'a Scope<'a>,
    var_vec: &'a Vector<'a>,
    index: usize,
}

impl<'a> Display for Var<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}[{}]", self.var_vec, self.index)
    }
}

impl<'a> Mul<Var<'a>> for i64 {
    type Output = &'a Term<'a>;

    fn mul(self, rhs: Var<'a>) -> Self::Output {
        rhs.scope.terms.imp_push(Term {
            scope: rhs.scope,
            coef: self,
            var: rhs,
        });
        &rhs.scope.terms[rhs.scope.terms.len() - 1]
    }
}

#[allow(unused_variables)]
fn main() {
    /// Breakdown of types
    fn break_down_of_types() {
        let scope = Scope::default();

        let x: &Vector = scope.new_var_vec("x");

        let x0: Var = x[0];
        let x1: Var = x[1];

        let t1: &Term = 3 * x[0];
        let t2: &Term = 4 * x[1];

        let le: &Expr = t1 + t2; // or
        let le: &Expr = 3 * x[0] + 4 * x[1];
    }

    /// Challenge #1
    /// x: VarVec being a symbolic vector of variables,
    /// we need x[i] operator to create a new value of the scalar type
    /// Var and return a reference to it for an arbitrary index i.
    fn challenge1() {
        let scope = Scope::default();

        let x: &Vector = scope.new_var_vec("x");

        let x0: Var = x[0];
        let x1: Var = x[1];
    }

    /// Challenge #2
    /// It is very important to have the symbolic types Var and Term
    /// to implement the Copy trait to achieve the desired expressive api.
    fn challenge2() {
        let scope = Scope::default();

        let x: &Vector = scope.new_var_vec("x");

        let t = 42 * x[0];
        let e1 = t + 3 * x[1];
        let e2 = t + 2 * x[0];
    }

    /// The Goal
    fn the_goal() {
        let scope = Scope::default();

        let x = scope.new_var_vec("x");

        let le = 3 * x[0] + 4 * x[1];

        assert_eq!(&le.to_string(), "3.x[0] + 4.x[1]");
        println!("{}", le);
    }

    break_down_of_types();
    challenge1();
    challenge2();
    the_goal();
}
