extern crate test;

use std::cmp::{max, min};
use std::sync::mpsc::Sender;
use std::time::SystemTime;
use std::{thread, time};
use threadpool::ThreadPool;

/// Summary of solving a batch of 3x+1 numbers
pub struct BatchSummary {
    /// Starting number
    pub start: usize,
    pub end: usize,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub max_steps: usize,
}

/// Recursive implementation of Collatz. Returns number of iterations to reach 1.
#[allow(dead_code)]
pub fn recursive(num: usize) -> usize {
    fn _recurse(num: usize, count: usize) -> (usize, usize) {
        if num == 1 {
            return (num, count);
        }
        // println!("{}", num);
        let new_num;
        if num % 2 == 0 {
            new_num = num / 2;
        } else {
            new_num = 3 * num + 1;
        }
        return _recurse(new_num, count + 1);
    }
    let (_, count) = _recurse(num, 0);
    count
}

/// Non-recursive implementation of Collatz
#[allow(dead_code)]
pub fn naive(num: usize) -> usize {
    let mut count: usize = 0;
    let mut num = num;
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = num * 3 + 1;
        }
        count += 1;
    }
    return count;
}

/// Shortcut implementation of Collatz
#[allow(dead_code)]
pub fn shortcut(num: usize) -> usize {
    let mut count: usize = 0;
    let mut num = num;
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = (num * 3 + 1) / 2;
            count += 1; // accounts for the skipped step
        }
        count += 1;
    }
    return count;
}

/// A faster version of the shortcut implementation
///
/// This implementation takes another shortcut. In this programs, we're
/// solving numbers sequentially. When solving N, we know that
/// solutions for numbers less than N have already been solved. Therefore,
/// if the algorithm returns a number less than N, we can exit as we know
/// that number has already been solved.
///
/// This messes up the `count` variable beyond recognition. It is kept
/// to keep the function signature the same, and ensure the compiler
/// doesn't get ahead of itself and optimize the function out of existence.
#[allow(dead_code)]
pub fn faster_shortcut(num: usize) -> usize {
    // Special case for this implementation. Can't get to < 1.
    if num == 1 {
        return 1;
    }
    let mut count: usize = 0;
    let mut curr_num = num;
    while curr_num >= num {
        if curr_num % 2 == 0 {
            curr_num = curr_num / 2;
        } else {
            curr_num = (curr_num * 3 + 1) / 2;
        }
        count += 1;
    }
    return count;
}

/// Solver entry point
pub fn solve(
    start: usize,
    end: usize,
    output_channel: Sender<BatchSummary>,
    batch_size: usize,
    threads: usize,
) {
    let mut batch_start = start;
    let pool = ThreadPool::new(threads);

    while end == 0 || batch_start < end {
        // If running forever, don't fill memory with waiting jobs.
        while end == 0 && pool.queued_count() > threads * 5000 {
            thread::sleep(time::Duration::from_millis(500));
        }

        let batch_end = match end {
            0 => batch_start + batch_size,
            _ => min(batch_start + batch_size, end),
        };
        let output_channel = output_channel.clone();
        pool.execute(move || {
            let mut max_steps = 0;
            let start_time = SystemTime::now();
            for num in batch_start..batch_end {
                // steps used it kind of interesting, but really i'm making sure
                // the compiler doesn't make this function call disappear.
                max_steps = max(max_steps, shortcut(num));
            }
            // Send a completion summary to the output channel
            output_channel
                .send(BatchSummary {
                    start: batch_start,
                    end: batch_end,
                    start_time: start_time,
                    end_time: SystemTime::now(),
                    max_steps: max_steps,
                })
                .expect("channel broken!");
        });
        batch_start = batch_end;
    }
    pool.join();
    /*
    let duration = start_time.elapsed().unwrap().as_millis();
    let solved = num - start;
    println!(
        "Solved {} in {}ms ({:.3} solves/s)",
        solved,
        duration,
        solved as f32 / duration as f32
    );
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use test::Bencher;

    static SOLVES: &'static [usize] = &[
        0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10,
        111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11,
        24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14,
        102, 22,
    ];
    static TEST_SIZE: usize = 5000;

    fn test_is_correct(f: fn(usize) -> usize) {
        for i in 0..SOLVES.len() {
            let res = f(i + 1);
            assert_eq!(res, SOLVES[i])
        }
    }

    fn test_performance(f: fn(usize) -> usize, start: usize, end: usize) {
        for n in start..end {
            f(n);
        }
    }

    #[test]
    fn test_recursive() {
        test_is_correct(recursive);
    }

    /// Recursive impl benchmark starting at 1
    #[bench]
    fn bench_recursive_small(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(recursive, start, start + TEST_SIZE));
    }

    /// Recursive impl benchmark starting at 1,000,000
    #[bench]
    fn bench_recursive_mid(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(recursive, start, start + TEST_SIZE));
    }

    /// Recursive impl benchmark starting at 1,000,000,000
    #[bench]
    fn bench_recursive_big(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(recursive, start, start + TEST_SIZE));
    }

    #[test]
    fn test_naive() {
        test_is_correct(naive);
    }

    /// Naive impl benchmark starting at 1
    #[bench]
    fn bench_naive_small(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(naive, start, start + TEST_SIZE));
    }

    /// Naive impl benchmark starting at 1,000,000
    #[bench]
    fn bench_naive_mid(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(naive, start, start + TEST_SIZE));
    }

    /// Naive impl benchmark starting at 1,000,000,000
    #[bench]
    fn bench_naive_big(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(naive, start, start + TEST_SIZE));
    }

    #[test]
    fn test_shortcut() {
        // N.B.: Ignored because the shortcut method has fewer steps per solve
        test_is_correct(shortcut);
    }

    /// Shortcut impl benchmark starting at 1
    #[bench]
    fn bench_shortcut_small(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(shortcut, start, start + TEST_SIZE));
    }

    /// Shortcut impl benchmark starting at 1,000,000
    #[bench]
    fn bench_shortcut_mid(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(shortcut, start, start + TEST_SIZE));
    }

    /// Shortcut impl benchmark starting at 1,000,000,000
    #[bench]
    fn bench_shortcut_big(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(shortcut, start, start + TEST_SIZE));
    }

    /// Faster shortcut impl benchmark starting at 1
    #[bench]
    fn bench_faster_shortcut_small(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(faster_shortcut, start, start + TEST_SIZE));
    }

    /// Faster shortcut impl benchmark starting at 1,000,000
    #[bench]
    fn bench_faster_shortcut_mid(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(faster_shortcut, start, start + TEST_SIZE));
    }

    /// Faster shortcut impl benchmark starting at 1,000,000,000
    #[bench]
    fn bench_faster_shortcut_big(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(faster_shortcut, start, start + TEST_SIZE));
    }

    fn test_solve_performance(start: usize, end: usize, b: &mut Bencher) {
        b.iter(|| {
            let (tx, rx): (Sender<BatchSummary>, Receiver<BatchSummary>) = mpsc::channel();
            // Smaller batch size than the default so that we're sure to actually use multiple threads
            solve(start, end, tx, 1000, 4)
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
}
