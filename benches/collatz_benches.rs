#![feature(test)]

extern crate collatz;
extern crate test;

use collatz::{bitwise, faster_shortcut, BatchSummary};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use test::Bencher;

static TEST_SIZE: usize = 5_000;

fn test_performance(f: fn(usize) -> usize, start: usize, end: usize) {
    // The "faster" method can skip even numbers since those can't start a cycle.
    if f == faster_shortcut || f == bitwise {
        let mut nums = start..end;
        if start % 2 == 0 {
            nums.next();
        }
        for n in nums.step_by(2) {
            f(n);
        }
    } else {
        for n in start..end {
            f(n);
        }
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

/// simple impl benchmark starting at 1
#[bench]
fn bench_simple_small(b: &mut Bencher) {
    let start = 1;
    b.iter(|| test_performance(collatz::simple, start, start + TEST_SIZE));
}

/// simple impl benchmark starting at 1,000,000
#[bench]
fn bench_simple_mid(b: &mut Bencher) {
    let start = 1_000_000;
    b.iter(|| test_performance(collatz::simple, start, start + TEST_SIZE));
}

/// simple impl benchmark starting at 1,000,000,000
#[bench]
fn bench_simple_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    b.iter(|| test_performance(collatz::simple, start, start + TEST_SIZE));
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

/// Faster shortcut impl benchmark starting at 1
#[bench]
fn bench_bitwise_small(b: &mut Bencher) {
    let start = 1;
    b.iter(|| test_performance(collatz::bitwise, start, start + TEST_SIZE));
}

/// Faster shortcut impl benchmark starting at 1,000,000
#[bench]
fn bench_bitwise_mid(b: &mut Bencher) {
    let start = 1_000_000;
    b.iter(|| test_performance(collatz::bitwise, start, start + TEST_SIZE));
}

/// Faster shortcut impl benchmark starting at 1,000,000,000
#[bench]
fn bench_bitwise_big(b: &mut Bencher) {
    let start = 1_000_000_000;
    b.iter(|| test_performance(collatz::bitwise, start, start + TEST_SIZE));
}

fn test_solve_performance(start: usize, end: usize, b: &mut Bencher) {
    b.iter(|| {
        let (tx, _): (Sender<BatchSummary>, Receiver<BatchSummary>) = mpsc::channel();
        collatz::solve(start, end, tx, 209, 24);
        // let (tx, _): (Sender<(usize, usize)>, Receiver<(usize, usize)>) = mpsc::channel();
        // collatz::solve_no_batching(start, end, tx, 24);
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
