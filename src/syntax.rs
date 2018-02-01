pub use span::{Symbol as Sy, Spanned as Sn};
use span::Span;
use std::str::FromStr;
use std::fmt::Debug;

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
        name: Sn<Sy>,
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
pub type Sxr = Sn<Expr>;
pub type Psxr = Ps<Expr>;

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
