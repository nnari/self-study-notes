#![feature(test)]
extern crate time;
use time::PreciseTime;
mod exercise1;

fn main() {
    let n = 100;
    exercise1::primesToN(n);
}
