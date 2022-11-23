extern crate test;

/// Recursive implementation of Collatz. Returns number of iterations to reach 1.
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
pub fn shortcut(num: usize) -> usize {
    let mut count: usize = 0;
    let mut num = num;
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = (num * 3 + 1)/2;
            count += 1; // accounts for the skipped step
        }
        count += 1;
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    static SOLVES:  &'static [usize] = &[0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17, 4, 12, 20, 20, 7, 7, 15, 15, 10, 23, 10, 111, 18, 18, 18, 106, 5, 26, 13, 13, 21, 21, 21, 34, 8, 109, 8, 29, 16, 16, 16, 104, 11, 24, 24, 24, 11, 11, 112, 112, 19, 32, 19, 32, 19, 19, 107, 107, 6, 27, 27, 27, 14, 14, 14, 102, 22];

    fn test_is_correct(f: fn(usize) -> usize) {
        for i in 0..SOLVES.len() {
            let res = f(i+1);
            assert_eq!(res, SOLVES[i])
        }
    }

    fn test_performance(f: fn(usize) -> usize) {
        for n in 1..5000 {
            f(n);
        }
    }

    #[test]
    fn test_recursive() {
        test_is_correct(recursive);
    }

    #[bench]
    fn bench_recursive(b: &mut Bencher) {
        b.iter(|| test_performance(recursive));
    }

    #[test]
    fn test_naive() {
        test_is_correct(naive);
    }

    #[bench]
    fn bench_naive(b: &mut Bencher) {
        b.iter(|| test_performance(naive));
    }

    #[test]
    fn test_shortcut() {
        // N.B.: Ignored because the shortcut method has fewer steps per solve
        test_is_correct(shortcut);
    }

    #[bench]
    fn bench_shortcut(b: &mut Bencher) {
        b.iter(|| test_performance(shortcut));
    }
}
