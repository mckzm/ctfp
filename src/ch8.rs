// <https://bartoszmilewski.com/2015/02/03/functoriality/>
// 5. Implement a bifunctor

#![allow(unused)]

trait Bifunctor {
    type InnerFirst;
    type InnerSecond;
    type OuterTarget<V, W>: Bifunctor;

    fn bimap<V, W, F: Fn(Self::InnerFirst) -> V, G: Fn(Self::InnerSecond) -> W>(
        self,
        f: F,
        g: G,
    ) -> Self::OuterTarget<V, W>
    where
        Self: Sized,
        Self::OuterTarget<V, Self::InnerSecond>: Bifunctor<
            InnerFirst = V,
            InnerSecond = Self::InnerSecond,
            OuterTarget<V, W> = Self::OuterTarget<V, W>,
        >,
    {
        self.first(f).second(g)
    }

    fn first<V, F: Fn(Self::InnerFirst) -> V>(
        self,
        f: F,
    ) -> Self::OuterTarget<V, Self::InnerSecond>
    where
        Self: Sized;
    fn second<W, G: Fn(Self::InnerSecond) -> W>(
        self,
        g: G,
    ) -> Self::OuterTarget<Self::InnerFirst, W>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
enum Either<T, U> {
    Left(T),
    Right(U),
}

impl<T, U> Bifunctor for Either<T, U> {
    type InnerFirst = T;
    type InnerSecond = U;
    type OuterTarget<V, W> = Either<V, W>;

    fn first<V, F: Fn(T) -> V>(self, f: F) -> Either<V, U> {
        match self {
            Either::Left(t) => Either::Left(f(t)),
            Either::Right(u) => Either::Right(u),
        }
    }

    fn second<W, G: Fn(U) -> W>(self, g: G) -> Either<T, W> {
        match self {
            Either::Left(t) => Either::Left(t),
            Either::Right(u) => Either::Right(g(u)),
        }
    }
}

impl<T, U> Bifunctor for (T, U) {
    type InnerFirst = T;
    type InnerSecond = U;
    type OuterTarget<V, W> = (V, W);

    fn first<V, F: Fn(T) -> V>(self, f: F) -> (V, U) {
        (f(self.0), self.1)
    }

    fn second<W, G: Fn(U) -> W>(self, g: G) -> (T, W) {
        (self.0, g(self.1))
    }
}

impl<T, E> Bifunctor for Result<T, E> {
    type InnerFirst = T;
    type InnerSecond = E;
    type OuterTarget<V, W> = Result<V, W>;

    fn first<V, F: Fn(T) -> V>(self, f: F) -> Result<V, E> {
        self.map(f)
    }

    fn second<W, G: Fn(E) -> W>(self, g: G) -> Result<T, W> {
        self.map_err(g)
    }
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn test_either_bifunctor() {
        let len = |s: &str| s.len();
        let count_ones = |x: usize| x.count_ones();

        let left = Either::Left("42");
        let left_up = left.bimap(len, count_ones);
        assert_eq!(left_up, Either::Left(2));

        let incr_to_u32 = |x: u8| (x - 1) as u32;
        let incr_to_u64 = |x: u32| (x + 1) as u64;

        let right = Either::Right(42u32);
        let right_up = right.bimap(incr_to_u32, incr_to_u64);
        assert_eq!(right_up, Either::Right(43u64));
    }

    #[test]
    fn test_pair_bifunctor() {
        let split = |s: &str| {
            s.split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };
        let incr_pair = |(a, b): (u32, u32)| ((a + 1) as u64, (b + 1) as u64);
        let v = vec![
            "First".to_string(),
            "second".to_string(),
            "third".to_string(),
        ];

        let pair = ("First second third", (0, 1));
        let uplifted = pair.bimap(split, incr_pair);
        assert_eq!(uplifted, (v, (1u64, 2u64)));
    }

    #[test]
    fn test_result_bifunctor() {
        let make_pair = |n: u32| (n, n + 1);
        let log = |e: ParseIntError| e.to_string() + " - with uplift";

        let ok: Result<u32, _> = "42".parse();
        let err = "Hello, World!".parse::<u32>();

        let ok = ok.bimap(make_pair, log);
        let err = err.bimap(make_pair, log);

        assert_eq!(ok, Ok((42u32, 43u32)));
        assert_eq!(
            err,
            Err("invalid digit found in string - with uplift".to_string())
        );
    }
}
