use std::collections::hash_map::{HashMap, Entry};
use std::hash::Hash;
use std::sync::Mutex;
use std::fmt::{Display, Debug, Formatter, Error as FmtError};
use std::path::{Path, PathBuf};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub struct Interner<V: Clone + Hash + Eq> {
    forward: HashMap<V, usize>,
    backward: Vec<V>
}

impl<V: Clone + Hash + Eq> Interner<V> {
    pub fn new() -> Interner<V> {
        Interner { forward: HashMap::new(), backward: Vec::new() }
    }

    pub fn intern(&mut self, v: V) -> usize {
        let e = self.forward.entry(v);
        match e {
            Entry::Vacant(e) => {
                let id = self.backward.len();
                self.backward.push(e.key().clone());
                e.insert(id);
                id
            }
            Entry::Occupied(e) => *e.get()
        }
    }

    pub fn get(&self, i: usize) -> &V {
        &self.backward[i]
    }
}

lazy_static! {
    static ref FILES: Mutex<Interner<PathBuf>> =
        Mutex::new(Interner::new());
    static ref SYMBOLS: Mutex<Interner<String>> =
        Mutex::new(Interner::new());
}

fn with_files<T, F: FnOnce(&mut Interner<PathBuf>) -> T>(f: F) -> T {
    f(&mut FILES.lock().expect("File interner poisoned!"))
}

fn with_symbols<T, F: FnOnce(&mut Interner<String>) -> T>(f: F) -> T {
    f(&mut SYMBOLS.lock().expect("Symbol interner poisoned!"))
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceFile(usize);

impl SourceFile {
    pub fn new(path: PathBuf) -> SourceFile {
        SourceFile(with_files(|i| i.intern(path)))
    }

    pub fn path(self) -> PathBuf {
        with_files(|i| i.get(self.0).clone())
    }
}

impl Display for SourceFile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.path().display())
    }
}

impl Debug for SourceFile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "SourceFile {{ {:?} }}", self.path().display())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Symbol(usize);

impl FromStr for Symbol {
    type Err = !;

    fn from_str(s: &str) -> Result<Symbol, !> {
        Ok(Symbol::new(s))
    }
}

impl Symbol {
    pub fn new(ident: &str) -> Symbol {
        Symbol(with_symbols(move |i| i.intern(ident.to_owned())))
    }

    pub fn ident(self) -> String {
        with_symbols(move |i| i.get(self.0).to_owned())
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.ident())
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Symbol({:?})", self.ident())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Span {
    pub source: SourceFile,
    pub start: usize,
    pub end: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}:{}..{}", self.source, self.start, self.end)
    }
}

impl Span {
    pub fn file<P: AsRef<Path>>(path: P, len: usize) -> Span {
        Span {
            source: SourceFile::new(path.as_ref().to_owned()),
            start: 0,
            end: len,
        }
    }

    pub fn on<T>(self, node: T) -> Spanned<T> {
        Spanned { node: node, span: self }
    }

    pub fn within(self, start: usize, end: usize) -> Span {
        Span {
            source: self.source,
            start: self.start + start,
            end: (self.start + end).min(self.end)
        }
    }

    pub fn union(self, other: Span) -> Span {
        assert_eq!(self.source, other.source);
        Span {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Spanned<T> {
    pub span: Span,
    pub node: T,
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.node.fmt(f)
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.node }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.node }
}

impl<T> Spanned<T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Spanned<U> {
        self.span.on(f(self.node))
    }

    pub fn on<U>(self, node: U) -> Spanned<U> {
        self.span.on(node)
    }
}
