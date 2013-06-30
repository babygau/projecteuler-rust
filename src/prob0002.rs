#[link(name = "prob0002", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::iterator::AdditiveIterator;
use common::extiter::Fibonacci;

pub static expected_answer: &'static str = "4613732";

pub fn solve() -> ~str {
    let limit = 4000000;
    return Fibonacci::new::<uint>()
        .take_while(|&f| f < limit)
        .filter(|&f| f % 2 == 0)
        .sum()
        .to_str();
}