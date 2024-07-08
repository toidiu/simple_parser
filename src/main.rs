//! A follow along of https://adriann.github.io/rust_parser.html to learn more about parsing.
//!
//! Goal is to be able to parse an expression like `12 + 34 * ( 56 + [ 7 ] )`

// TODO: next start at the Parsing section.

#![allow(unused)]

use std::env;

mod error;
mod grammar;
mod lexer;
mod parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        let tree = parser::parse(&args[1]).unwrap();

        println!("{}\n{:?}", parser::format_pretty(&tree), tree);
    }
}
