#![feature(rustc_private)]

extern crate syntax_pos;

use std::io::prelude::*;
use std::fs::File;
use std::env;

mod syntax {
    include!(concat!(env!("OUT_DIR"), "/syntax.rs"));
}

fn main() {
    let fname = env::args()
        .nth(1)
        .expect("No input file given: example [dust file]");
    let mut f = File::open(fname).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("{:?}", syntax::code(&s));
}
