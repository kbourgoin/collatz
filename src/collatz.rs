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
        match num {
            1 => (num, count),
            num if num % 2 == 0 => _recurse(num / 2, count + 1),
            _ => _recurse(3 * num + 1, count + 1),
        }
    }
    _recurse(num, 0).1
}

/// Non-recursive implementation of Collatz
#[allow(dead_code)]
pub fn naive(num: usize) -> usize {
    let (mut num, mut count) = (num, 0);
    while num != 1 {
        (num, count) = match num {
            num if num % 2 == 0 => (num / 2, count + 1),
            _ => (3 * num + 1, count + 1),
        };
    }
    count
}

/// Shortcut implementation of Collatz
#[allow(dead_code)]
pub fn shortcut(num: usize) -> usize {
    let (mut num, mut count) = (num, 0);
    while num != 1 {
        (num, count) = match num {
            num if num % 2 == 0 => (num / 2, count + 1),
            _ => ((3 * num + 1) / 2, count + 2), // +2 accounts for skipped step
        };
    }
    count
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
    let mut count = 0;
    let mut num = num;
    let starting_num = num;
    while num >= starting_num {
        (num, count) = match num {
            num if num % 2 == 0 => (num / 2, count + 1),
            _ => ((3 * num + 1) / 2, count + 2),
        };
    }
    return count as usize;
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
                // max steps is kind of interesting, but really i'm making sure
                // the compiler doesn't make this function call disappear.
                max_steps = max(max_steps, faster_shortcut(num));
            }
            // Send a completion summary to the output channel
            output_channel
                .send(BatchSummary {
                    start: batch_start,
                    end: batch_end,
                    start_time,
                    end_time: SystemTime::now(),
                    max_steps,
                })
                .expect("channel broken!");
        });
        batch_start = batch_end;
    }
    pool.join();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    static ANSWERS: &'static [usize] = &[
        0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10,
        111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11,
        24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14,
        102, 22,
    ];

    // Generated test data from running faster implementation. Ensures answers don't change, but
    // isn't validated to be correct.
    static FASTER_ANSWERS: &'static [usize] = &[
        1, 1, 6, 1, 3, 1, 11, 1, 3, 1, 8, 1, 3, 1, 11, 1, 3, 1, 6, 1, 3, 1, 8, 1, 3, 1, 96, 1, 3,
        1, 91, 1, 3, 1, 6, 1, 3, 1, 13, 1, 3, 1, 8, 1, 3, 1, 88, 1, 3, 1, 6, 1, 3, 1, 8, 1, 3, 1,
        11, 1, 3, 1, 88, 1, 3, 1, 6, 1, 3, 1, 83, 1, 3, 1, 8, 1, 3, 1, 13, 1, 3, 1, 6, 1, 3, 1, 8,
        1, 3, 1, 73, 1, 3, 1, 13, 1, 3, 1, 6, 1,
    ];

    fn test_is_correct(f: fn(usize) -> usize, answers: &'static [usize]) {
        for i in 0..answers.len() {
            let res = f(i + 1);
            assert_eq!(res, answers[i])
        }
    }

    #[test]
    fn test_recursive() {
        test_is_correct(recursive, ANSWERS);
    }

    #[test]
    fn test_naive() {
        test_is_correct(naive, ANSWERS);
    }

    #[test]
    fn test_shortcut() {
        test_is_correct(shortcut, ANSWERS);
    }

    #[test]
    fn test_faster_shortcut() {
        test_is_correct(faster_shortcut, FASTER_ANSWERS);
    }

    #[test]
    fn test_solve() {
        let (tx, rx): (Sender<BatchSummary>, Receiver<BatchSummary>) = mpsc::channel();
        solve(1, 100, tx, 10, 4);
        let tmp: Vec<BatchSummary> = rx.iter().collect();

        // For now, just be sure all 10 batches came out the other side
        assert_eq!(tmp.len(), 10);
    }
}
