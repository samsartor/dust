pub use span::{Symbol as Sy, Spanned as Sn};
use span::Span;
use std::str::FromStr;
use std::fmt::Debug;
use std::boxed::FnBox;

pub type Ps<T> = Box<Sn<T>>;

#[derive(Debug)]
pub enum Ty {
    Path {
        abs: bool,
        path: Vec<Sn<Sy>>,
    },
    Tuple(Vec<Sn<Ty>>),
}

impl Ty {
    pub fn unit() -> Ty {
        Ty::Tuple(Vec::new())
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BLeft,
    BRight,
    BAnd,
    BOr,
    BXor,
    And,
    Or,
    Eq,
    Neq,
    Less,
    Greater,
    Leq,
    Geq,
    Assign,
    Then,
    Else,
    Index,
}

#[derive(Debug)]
pub enum UnaryOp {
    Ref {
        mutable: bool,
        lifetime: Option<Sy>,
    },
    Star,
    Not,
    Neg,
    Try,
}

#[derive(Debug)]
pub enum NumTy {
    Signed(u16),
    Unsigned(u16),
    Float(u16),
}

#[derive(Debug)]
pub enum Pattern {
    /// Pattern used by `if` statements
    True,
    /// `$label Ty`
    Lot {
        label: Option<Sn<Sy>>,
        ty: Option<Sn<Ty>>,
    },
    Or(Vec<Sn<Pattern>>),
    And(Vec<Sn<Pattern>>),
}

#[derive(Debug)]
pub enum Expr {
    Ident(Sy),
    Num {
        value: Sy,
        ty: Option<NumTy>,
    },
    Guard {
        val: Psxr,
        pat: Sn<Pattern>,
    },
    Member {
        on: Psxr,
        names: Vec<Sn<Sy>>,
    },
    Call {
        on: Psxr,
        args: Vec<Sxr>,
    },
    Binop {
        left: Psxr,
        op: BinaryOp,
        right: Psxr,
    },
    Unop {
        op: UnaryOp,
        right: Psxr,
    },
    Block(Vec<Sxr>),
    Unit,
}

impl Expr {
    pub fn set_left(&mut self, to: Psxr) {
        use self::Expr::*;
        match *self {
            Guard { ref mut val, .. } => *val = to,
            Member { ref mut on, .. } => *on = to,
            Call { ref mut on, .. } => *on = to,
            Binop { ref mut left, .. } => *left = to,
            Unop { ref mut right, .. } => *right = to,
            _ => (),
        }
    }
}

pub type Xr = Expr;
pub type Sxr = Sn<Expr>;
pub type Psxr = Ps<Expr>;

pub type Rhs = Box<FnBox(Sxr) -> Xr>;
macro_rules! rhs {
    ($x:ident => $f:expr) => {{
        let v: Rhs = Box::new(|$x| $f);
        v
    }};
}

// Builds a binary operation
pub fn binop(l: Sxr, op: BinaryOp, r: Sxr) -> Sxr {
    Sn {
        span: l.span.union(r.span),
        node: Expr::Binop {
            left: Box::new(l),
            op,
            right: Box::new(r) },
    }
}

// Builds a unary operation
pub fn unop(mark: Span, op: UnaryOp, r: Sxr) -> Sxr {
    Sn {
        span: mark.union(r.span),
        node: Expr::Unop {
            op,
            right: Box::new(r) },
    }
}

// FromStr wrapper, panics if invalid
pub fn froms<T: FromStr>(s: &str) -> T where T::Err : Debug {
    FromStr::from_str(s).unwrap()
}
