use std::cmp::min;
use std::sync::mpsc::Sender;
use std::time::{Duration, SystemTime};
use std::{thread, time};
use threadpool::ThreadPool;

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

/// Solver entry point
#[allow(dead_code)]
pub fn solve(
    implementation: fn(usize) -> usize,
    start: usize,
    end: usize,
    output_channel: Sender<(usize, usize)>,
    threads: usize,
) {
    match threads {
        1 => solve_st(implementation, start, end, output_channel),
        _ => solve_mt(implementation, start, end, output_channel, threads),
    }
}

fn solve_mt(
    implementation: fn(usize) -> usize,
    start: usize,
    end: usize,
    output_channel: Sender<(usize, usize)>,
    threads: usize,
) {
    let start_time = SystemTime::now();
    let mut num = start;
    let pool = ThreadPool::new(threads);

    const batch_size: usize = 1000;

    while end == 0 || num < end {
        // Make sure we don't explode memory with non-running jobs.
        while pool.queued_count() > threads * 5000 {
            thread::sleep(time::Duration::from_millis(50));
        }
        let batch_end = min(num + batch_size, end);
        // let output_channel = output_channel.clone();
        pool.execute(move || {
            for num in num..batch_end {
                let result = implementation(num);
                // TODO: Make this configurable or move to receiver.
                // Print every million to saturate CPU and not I/O
                let solved = num + 1 - start;
                if solved % 10_000_000 == 0 {
                    println!("{:e}: {}", num, result);
                    // output_channel.send((num, result)).unwrap();
                    let duration = start_time.elapsed().unwrap().as_millis();
                    println!(
                        "Solved {} in {}ms ({:.3} solves/s)",
                        solved,
                        duration,
                        solved as f32 / duration as f32
                    );
                }
            }
        });
        num = batch_end;
    }
    pool.join();
    let duration = start_time.elapsed().unwrap().as_millis();
    let solved = num - start;
    println!(
        "Solvea {} in {}ms ({:.3} solves/s)",
        solved,
        duration,
        solved as f32 / duration as f32
    );
}

fn solve_st(
    implementation: fn(usize) -> usize,
    start: usize,
    end: usize,
    output_channel: Sender<(usize, usize)>,
) {
    // Simpler single-threaded version
    let mut num = start;
    while end == 0 || num < end {
        let result = implementation(num);
        // TODO: Make this configurable or move to receiver.
        // Print every million so we saturate CPU and on I/O
        if num % 1_000_000 == 0 {
            output_channel.send((num, result)).unwrap();
        }
        num += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    static SOLVES: &'static [usize] = &[
        0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10,
        111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11,
        24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14,
        102, 22,
    ];

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

    /// Recursive impl benchmark (1..5000)
    #[bench]
    fn small_bench_recursive(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(recursive, start, start + 5000));
    }

    /// Recursive impl benchmark (1,000,000..1,005,000)
    #[bench]
    fn mid_bench_recursive(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(recursive, start, start + 5000));
    }

    /// Recursive impl benchmark (1,000,000,000..1,000,005,000)
    #[bench]
    fn big_bench_recursive(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(recursive, start, start + 5000));
    }

    #[test]
    fn test_naive() {
        test_is_correct(naive);
    }

    /// Naive impl benchmark (1..5000)
    #[bench]
    fn small_bench_naive(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(naive, start, start + 5000));
    }

    /// Naive impl benchmark (1,000,000..1,005,000)
    #[bench]
    fn mid_bench_naive(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(naive, start, start + 5000));
    }

    /// Naive impl benchmark (1,000,000,000..1,000,005,000)
    #[bench]
    fn big_bench_naive(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(naive, start, start + 5000));
    }

    #[test]
    fn test_shortcut() {
        // N.B.: Ignored because the shortcut method has fewer steps per solve
        test_is_correct(shortcut);
    }

    /// Shortcut impl benchmark (1..5000)
    #[bench]
    fn small_bench_shortcut(b: &mut Bencher) {
        let start = 1;
        b.iter(|| test_performance(shortcut, start, start + 5000));
    }

    /// Shortcut impl benchmark (1,000,000..1,005,000)
    #[bench]
    fn mid_bench_shortcut(b: &mut Bencher) {
        let start = 1_000_000;
        b.iter(|| test_performance(shortcut, start, start + 5000));
    }

    /// Shortcut impl benchmark (1,000,000,000..1,000,005,000)
    #[bench]
    fn big_bench_shortcut(b: &mut Bencher) {
        let start = 1_000_000_000;
        b.iter(|| test_performance(shortcut, start, start + 5000));
    }
}
