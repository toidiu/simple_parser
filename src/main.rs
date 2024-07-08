//! A follow along of https://adriann.github.io/rust_parser.html to learn more about parsing.
//!
//! Goal is to be able to parse an expression like `12 + 34 * ( 56 + [ 7 ] )`

#![allow(unused)]

mod error;
mod grammar;
mod lexing;

fn main() {
    println!("Hello, world!");
}
