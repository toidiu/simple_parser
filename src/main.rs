//! A follow along of https://adriann.github.io/rust_parser.html to learn more about parsing.
//!
//! Goal is to be able to parse an expression like `12 + 34 * ( 56 + [ 7 ] )`

// TODO: next start at the Parsing section.

#![allow(unused)]

use std::env;

mod error;
mod grammar;
mod lexing;

fn main() {
    // println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is {}", args[1]);
        println!("{:?}", parse(&args[1]));
    }
}

// Not sure how exactly to read this grammar. Need to understand the notation. Probably at
// http://pages.cs.wisc.edu/~fischer/cs536.s08/course.hold/html/NOTES/3.CFG.html#exp
//
// > The grammar I came up with is as follows:
// >    expr -> summand + expr | summand
// >    summand -> term * summand | term
// >    term -> NUMBER | ( expr )
fn parse(input: &str) {}

fn parse_expr() {}

fn parse_summand() {}

fn parse_term() {}
