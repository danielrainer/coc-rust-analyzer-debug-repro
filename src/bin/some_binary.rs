#![feature(test)]

use test::Bencher;

extern crate test;

fn main() {
    println!("Hello from some binary.");
}

#[bench]
fn bench_some_binary(b: &mut Bencher) {
    println!("Hello from bench in some binary.");
}
