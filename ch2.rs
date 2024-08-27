// <https://bartoszmilewski.com/2014/11/24/types-and-functions/>
// 1. implement memoize
// 5. implement all Bool -> Bool funcs

#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

// 1. memoize
// using a wrapper struct allows us to inspect the cache
// it tanks perf though
struct Memoizer<T, U> {
    f: Box<dyn Fn(T) -> U>,
    store: HashMap<T, U>,
}

impl<T: Clone + Eq + Hash, U: Clone> Memoizer<T, U> {
    fn new(f: impl Fn(T) -> U + 'static) -> Self {
        Self {
            f: Box::new(f),
            store: HashMap::new(),
        }
    }

    // we just wrap the initial call to `f`;
    // recursive functions' intermediate results are not cached
    fn call(&mut self, arg: T) -> U {
        self.store
            .entry(arg.clone())
            .or_insert((self.f)(arg))
            .clone()
    }
}

// 5. Bool -> Bool funcs
fn id_bool(b: bool) -> bool {
    b
}
fn negate(b: bool) -> bool {
    !b
}
fn always_true(_: bool) -> bool {
    true
}
fn always_false(_: bool) -> bool {
    false
}
// cheeky: _|_ = divergent type
fn diverge_bool(_: bool) -> ! {
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn gen_fib_nums_to_ten() -> HashMap<u32, u32> {
        HashMap::from([
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (7, 13),
            (8, 21),
            (9, 34),
            (10, 55),
        ])
    }

    fn fib(n: u32) -> u32 {
        match n {
            0 | 1 => n,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    #[test]
    fn test_fibonacci_correctness() {
        let mut fib_memoized = Memoizer::new(fib);
        assert_eq!(8, fib_memoized.call(6));
    }

    #[test]
    fn test_memoized_fibonacci_store() {
        let mut fib_memoized = Memoizer::new(fib);

        for i in 0..=10 {
            fib_memoized.call(i);
        }

        assert_eq!(fib_memoized.store, gen_fib_nums_to_ten());
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
        let mut fib_memoized = Memoizer::new(fib);
        let mut d: Duration = Default::default();

        for _ in 0..1000 {
            let start = Instant::now();
            for i in 0..=30 {
                fib_memoized.call(i);
            }
            d += start.elapsed();
        }

        println!("{:?}", d / 1000);
    }

    #[ignore]
    #[test]
    fn bench_fib_memoized_warmed_up() {
        let mut fib_memoized = Memoizer::new(fib);
        let mut d: Duration = Default::default();

        for i in 0..=30 {
            fib_memoized.call(i);
        }

        for _ in 0..1000 {
            let start = Instant::now();
            for i in 0..=30 {
                fib_memoized.call(i);
            }
            d += start.elapsed();
        }

        println!("{:?}", d / 1000);
    }
}
