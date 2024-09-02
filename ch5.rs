// <https://bartoszmilewski.com/2015/01/07/products-and-coproducts/>
// 4. impl the equivalent of Haskell's `Either` type as a generic type
// <https://hackage.haskell.org/package/base/docs/Data-Either.html>

#![allow(unused)]

enum Either<T, U> {
    Left(T),
    Right(U),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[track_caller]
    fn test_either() {
        let a: Either<u32, String> = Either::Left(5);
        let b: Either<f64, &str> = Either::Right("b");
        let nested: Either<Either<bool, u32>, bool> = Either::Left(Either::Right(42u32));

        let extracted = match nested {
            Either::Left(Either::Right(v)) => v,
            _ => 0,
        };

        assert_eq!(42u32, extracted);
    }
}
