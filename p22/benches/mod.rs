#![feature(test)]
extern crate test;

use test::{black_box, Bencher};

use p22::calc::celsius2farenheit;

#[bench]
fn bench_test_celsius2farenheit(b: &mut Bencher) {
    b.iter(|| {
        let x: i32 = black_box(-40);
        let y: i32 = celsius2farenheit(x);
        black_box(y);
    })
}

use p22::calc::farenheit2celsius;

#[bench]
fn bench_test_farenheit2celsius(b: &mut Bencher) {
    b.iter(|| {
        let x: i32 = black_box(-40);
        let y: i32 = farenheit2celsius(x);
        black_box(y);
    })
}

use p22::calc::fibonacci_loop;

#[bench]
fn bench_test_fibonacci_loop(b: &mut Bencher) {
    b.iter(|| {
        let x: u32 = black_box(30);
        let y: u32 = fibonacci_loop(x);
        black_box(y);
    })
}

use p22::calc::fibonacci_rec;

#[bench]
fn bench_test_fibonacci_rec(b: &mut Bencher) {
    b.iter(|| {
        let x: u32 = black_box(30);
        let y: u32 = fibonacci_rec(x);
        black_box(y);
    })
}
