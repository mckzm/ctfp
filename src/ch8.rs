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

#[cfg(test)]
mod tests {
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
}
