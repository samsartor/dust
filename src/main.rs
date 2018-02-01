#![feature(rustc_private)]
#![feature(never_type)]

#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;
use std::env;

pub mod syntax;
pub mod span;
pub mod parse {
    include!(concat!(env!("OUT_DIR"), "/parse.rs"));
}

fn main() {
    let path: PathBuf = env::args()
        .nth(1)
        .expect("No input file given: example [dust file]")
        .into();
    let mut file = File::open(&path).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    let span = span::Span::file(path, source.len());


    println!("{:#?}", parse::code(&source, span));
}
