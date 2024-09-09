// <https://bartoszmilewski.com/2015/01/20/functors/>
// 3. Implement the reader functor

#![allow(unused)]

fn reader_functor<T, U, V>(f: impl Fn(T) -> U, g: impl Fn(V) -> T) -> impl FnMut(V) -> U {
    move |v: V| f(g(v))
}

// RPITIT `impl GatLessFunctor` is opaque - we can't do much with it.
// By extension we can't do much w/ GatLessFunctor, though the code
// below compiles just fine.
trait GatLessFunctor<T> {
    fn gf_fmap<U>(self, f: impl Fn(T) -> U) -> impl GatLessFunctor<U>;
}

impl<T> GatLessFunctor<T> for Option<T> {
    fn gf_fmap<U>(self, f: impl Fn(T) -> U) -> impl GatLessFunctor<U> {
        self.map(f)
    }
}

// F could (should?) be FnMut instead of Fn
trait Functor {
    type InnerSource;
    type OuterTarget<U>: Functor;

    fn fmap<F: Fn(Self::InnerSource) -> U, U>(self, f: F) -> Self::OuterTarget<U>;
}

impl<T> Functor for Option<T> {
    type InnerSource = T;
    type OuterTarget<U> = Option<U>;

    fn fmap<F: Fn(T) -> U, U>(self, f: F) -> Option<U> {
        self.map(f)
    }
}

#[cfg(test)]
mod tests {
    use super::reader_functor;

    use super::*;

    #[test]
    fn test_reader_functor_u8_u32_u64() {
        let f = |r: u32| r as u64;
        let g = |a: u8| a as u32;
        let mut rf = reader_functor(f, g);

        assert_eq!(rf(8u8), 8u64);
    }

    #[test]
    fn test_maybe_functor() {
        let a = Some(42u8);
        let f = |a: u8| a as u32;
        let fmapped = a.fmap(f);

        assert_eq!(fmapped, Some(42u32));
        assert_eq!(None.fmap(f), None);
    }
}
