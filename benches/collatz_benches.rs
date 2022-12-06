#![feature(test)]

extern crate collatz;
extern crate test;

use collatz::BatchSummary;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use test::Bencher;

static TEST_SIZE: usize = 5000;

fn test_performance(f: fn(usize) -> usize, start: usize, end: usize) {
    for n in start..end {
        f(n);
    }
}

/// Recursive impl benchmark starting at 1
#[bench]
fn bench_recursive_small(b: &mut Bencher) {
    let start = 1;
    b.iter(|| test_performance(collatz::recursive, start, start + TEST_SIZE));
}

/// Recursive impl benchmark starting at 1,000,000
#[bench]
fn bench_recursive_mid(b: &mut Bencher) {
    let start = 1_000_000;
    b.iter(|| test_performance(collatz::recursive, start, start + TEST_SIZE));
}

/// Recursive impl benchmark starting at 1,000,000,000
#[bench]
fn bench_recursive_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    b.iter(|| test_performance(collatz::recursive, start, start + TEST_SIZE));
}

/// Naive impl benchmark starting at 1
#[bench]
fn bench_naive_small(b: &mut Bencher) {
    let start = 1;
    b.iter(|| test_performance(collatz::naive, start, start + TEST_SIZE));
}

/// Naive impl benchmark starting at 1,000,000
#[bench]
fn bench_naive_mid(b: &mut Bencher) {
    let start = 1_000_000;
    b.iter(|| test_performance(collatz::naive, start, start + TEST_SIZE));
}

/// Naive impl benchmark starting at 1,000,000,000
#[bench]
fn bench_naive_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    b.iter(|| test_performance(collatz::naive, start, start + TEST_SIZE));
}

/// Shortcut impl benchmark starting at 1
#[bench]
fn bench_shortcut_small(b: &mut Bencher) {
    let start = 1;
    b.iter(|| test_performance(collatz::shortcut, start, start + TEST_SIZE));
}

/// Shortcut impl benchmark starting at 1,000,000
#[bench]
fn bench_shortcut_mid(b: &mut Bencher) {
    let start = 1_000_000;
    b.iter(|| test_performance(collatz::shortcut, start, start + TEST_SIZE));
}

/// Shortcut impl benchmark starting at 1,000,000,000
#[bench]
fn bench_shortcut_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    b.iter(|| test_performance(collatz::shortcut, start, start + TEST_SIZE));
}

/// Faster shortcut impl benchmark starting at 1
#[bench]
fn bench_faster_shortcut_small(b: &mut Bencher) {
    let start = 1;
    b.iter(|| test_performance(collatz::faster_shortcut, start, start + TEST_SIZE));
}

/// Faster shortcut impl benchmark starting at 1,000,000
#[bench]
fn bench_faster_shortcut_mid(b: &mut Bencher) {
    let start = 1_000_000;
    b.iter(|| test_performance(collatz::faster_shortcut, start, start + TEST_SIZE));
}

/// Faster shortcut impl benchmark starting at 1,000,000,000
#[bench]
fn bench_faster_shortcut_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    b.iter(|| test_performance(collatz::faster_shortcut, start, start + TEST_SIZE));
}

fn test_solve_performance(start: usize, end: usize, b: &mut Bencher) {
    b.iter(|| {
        let (tx, _): (Sender<BatchSummary>, Receiver<BatchSummary>) = mpsc::channel();
        // Smaller batch size than the default so that we're sure to actually use multiple threads
        collatz::solve(start, end, tx, 1000, 4)
    });
}

/// Multithreaded solve benchmark starting at 1
#[bench]
fn bench_solve_small(b: &mut Bencher) {
    let start = 1;
    test_solve_performance(start, start + TEST_SIZE, b);
}

/// Multithreaded solve benchmark starting at 1,000,000
#[bench]
fn bench_solve_mid(b: &mut Bencher) {
    let start = 1_000_000;
    test_solve_performance(start, start + TEST_SIZE, b);
}

/// Multithreaded solve benchmark (1,000,000,000..1,000,005,000)
#[bench]
fn bench_solve_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    test_solve_performance(start, start + TEST_SIZE, b);
}
