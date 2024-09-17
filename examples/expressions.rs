use orx_imp_vec::*;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

/// A scope for expressions.
#[derive(Default)]
struct Scope<'a> {
    expressions: ImpVec<Expr<'a>>,
}

impl<'a> Scope<'a> {
    /// Bottom of the expressions recursion, the symbol primitive
    fn symbol(&'a self, name: &'static str) -> ExprInScope<'a> {
        let expr = Expr::Symbol(name);
        self.expressions.imp_push(expr);
        ExprInScope {
            scope: self,
            expr: &self.expressions[self.expressions.len() - 1],
        }
    }
}

/// A recursive expression with three demo variants
enum Expr<'a> {
    Symbol(&'static str),
    Addition(&'a Expr<'a>, &'a Expr<'a>),
    Subtraction(&'a Expr<'a>, &'a Expr<'a>),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Symbol(x) => write!(f, "{}", x),
            Expr::Addition(x, y) => write!(f, "{} + {}", x, y),
            Expr::Subtraction(x, y) => write!(f, "{} - {}", x, y),
        }
    }
}

/// Expression in a scope:
/// * it knows what it is
/// * it knows which scope it belongs to
///
/// It can implement Copy which turns out to be extremely important!
#[derive(Clone, Copy)]
struct ExprInScope<'a> {
    scope: &'a Scope<'a>,
    expr: &'a Expr<'a>,
}

impl<'a> ExprInScope<'a> {
    /// Recall, it knows that scope it belongs to,
    /// and can check it in O(1)
    fn belongs_to_same_scope(&self, other: Self) -> bool {
        let self_scope = self.scope as *const Scope;
        let other_scope = other.scope as *const Scope;
        self_scope == other_scope
    }
}
impl<'a> Display for ExprInScope<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

impl<'a> Add for ExprInScope<'a> {
    type Output = ExprInScope<'a>;

    /// We can create an expression by adding two expressions
    ///
    /// Where do we store the new expression?
    ///
    /// Of course, in the scope that both expressions belong to.
    /// And we can do so by `imp_push`.
    ///
    /// # Panics
    ///
    /// Panics if the lhs & rhs do not belong to the same scope.
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.belongs_to_same_scope(rhs));
        let expressions = &self.scope.expressions;
        let expr = Expr::Addition(self.expr, rhs.expr);
        expressions.imp_push(expr);
        ExprInScope {
            scope: self.scope,
            expr: &expressions[expressions.len() - 1],
        }
    }
}

impl<'a> Sub for ExprInScope<'a> {
    type Output = ExprInScope<'a>;

    /// Similarly, we can create an expression by subtracting two expressions
    ///
    /// Where do we store the new expression?
    ///
    /// Of course, in the scope that both expressions belong to.
    /// And we can do so by `imp_push`.
    ///
    /// # Panics
    ///
    /// Panics if the lhs & rhs do not belong to the same scope.
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.belongs_to_same_scope(rhs));
        let expressions = &self.scope.expressions;
        let expr = Expr::Subtraction(self.expr, rhs.expr);
        expressions.imp_push(expr);
        ExprInScope {
            scope: self.scope,
            expr: &expressions[expressions.len() - 1],
        }
    }
}

fn main() {
    let scope = Scope::default();

    // instantiate some symbols
    let x = scope.symbol("x");
    let y = scope.symbol("y");
    assert_eq!(&x.to_string(), "x");
    assert_eq!(&y.to_string(), "y");

    // apply binary operations to create new symbols
    let p = x + y;
    assert_eq!(&p.to_string(), "x + y");

    let q = x - y;
    assert_eq!(&q.to_string(), "x - y");

    // and further binary operations
    let t = p + q;
    assert_eq!(&t.to_string(), "x + y + x - y");

    // we only use 'scope' to create symbols
    // but in the background, all expressions are collected in our scope
    let all_expressions: Vec<_> = scope.expressions.iter().map(|x| x.to_string()).collect();
    assert_eq!(
        all_expressions,
        ["x", "y", "x + y", "x - y", "x + y + x - y"]
    );
}
