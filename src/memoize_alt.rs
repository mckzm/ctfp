//   polymorphic memoize implemented as an FnMut closure
//   drawback: no (simple) way to inspect the cache as closures are opaque

#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

fn memoize<T: Clone + PartialEq + Eq + Hash, U: Clone>(f: impl Fn(T) -> U) -> impl FnMut(T) -> U {
    let mut map: HashMap<T, U> = HashMap::new();
    move |x: T| map.entry(x).or_insert_with_key(|x| f(x.clone())).clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn fib(n: u32) -> u32 {
        match n {
            0 | 1 => n,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    #[test]
    fn test_fibonacci_correctness() {
        let mut fib_memoized = memoize(fib);
        assert_eq!(8, fib_memoized(6));
    }

    // naive pseudo-benchmarks for ballpark figures in lieu of using either
    // Unstable's test::Benchmark or Criterion.
    // [should really be macro_rules!()'ed to reduce duplication]
    // run w/ `cargo test -- --include-ignored --show-output`
    #[ignore]
    #[test]
    fn bench_fib() {
        let mut d: Duration = Default::default();

        for _ in 0..1000 {
            let start = Instant::now();
            for i in 0..=30 {
                fib(i);
            }
            d += start.elapsed();
        }

        println!("{:?}", d / 1000);
    }

    #[ignore]
    #[test]
    fn bench_fib_memoized_no_warmup() {
        let mut fib_memoized = memoize(fib);
        let mut d: Duration = Default::default();

        for _ in 0..1000 {
            let start = Instant::now();
            for i in 0..=30 {
                fib_memoized(i);
            }
            d += start.elapsed();
        }

        println!("{:?}", d / 1000);
    }

    #[ignore]
    #[test]
    fn bench_fib_memoized_warmed_up() {
        let mut fib_memoized = memoize(fib);
        let mut d: Duration = Default::default();

        for i in 0..=30 {
            fib_memoized(i);
        }

        for _ in 0..1000 {
            let start = Instant::now();
            for i in 0..=30 {
                fib_memoized(i);
            }
            d += start.elapsed();
        }

        println!("{:?}", d / 1000);
    }
}
