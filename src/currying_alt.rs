// After:
// <https://bartoszmilewski.com/2015/03/13/function-types/#:~:text=Currying>
// <https://www.kurtlawrence.info/blog/category-theory-with-rust-pt2#:~:text=Currying>

#![allow(unused)]

trait IsoCurry<T, U, V> {
    fn curry(&self, t: T) -> impl Fn(U) -> V;
    fn curry_once(self, t: T) -> impl FnOnce(U) -> V;

    fn uncurry(&self) -> impl Fn((T, U)) -> V;
    fn uncurry_once(self) -> impl FnOnce((T, U)) -> V;
}

impl<T, U, V, F> IsoCurry<T, U, V> for F
where
    T: Clone,
    F: Fn(T, U) -> V + Clone,
{
    fn curry(&self, t: T) -> impl Fn(U) -> V {
        let f = self.clone();
        move |u: U| f(t.clone(), u)
    }

    fn curry_once(self, t: T) -> impl FnOnce(U) -> V {
        move |u: U| self(t, u)
    }

    fn uncurry(&self) -> impl Fn((T, U)) -> V {
        let f = self.clone();
        move |(t, u)| f(t, u)
    }

    fn uncurry_once(self) -> impl FnOnce((T, U)) -> V {
        move |(t, u)| self(t, u)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    fn count_char(s: &str, c: char) -> usize {
        s.chars().filter(|ch| ch == &c).count()
    }

    #[test]
    fn test_curry() {
        let add = |x: u32, y: u32| x + y;
        let curried_add = add.curry(1);
        assert_eq!(curried_add(2), add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let curried_add = add.curry("Hello");
        assert_eq!(curried_add(", World!"), add("Hello", ", World!"));

        let curried_count_char = count_char.curry("element");
        assert_eq!(curried_count_char('e'), count_char("element", 'e'));
    }

    #[test]
    fn test_curry_once() {
        let add = |x: u32, y: u32| x + y;
        let curried_add = add.curry_once(1);
        assert_eq!(curried_add(2), add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let curried_add = add.curry_once("Hello");
        assert_eq!(curried_add(", World!"), add("Hello", ", World!"));

        let curried_count_char = count_char.curry_once("element");
        assert_eq!(curried_count_char('e'), count_char("element", 'e'));
    }

    #[test]
    fn test_uncurry() {
        let add = |x: u32, y: u32| x + y;
        let uncurried_add = add.uncurry();
        assert_eq!(uncurried_add((1, 2)), add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let uncurried_add = add.uncurry();
        assert_eq!(
            uncurried_add(("Hello", ", World!")),
            "Hello, World!".to_string()
        );

        let uncurried_count_char = count_char.uncurry();
        assert_eq!(
            uncurried_count_char(("element", 'e')),
            count_char("element", 'e')
        );
    }

    #[test]
    fn test_uncurry_once() {
        let add = |x: u32, y: u32| x + y;
        let uncurried_add = add.uncurry_once();
        assert_eq!(uncurried_add((1, 2)), add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let uncurried_add = add.uncurry_once();
        assert_eq!(
            uncurried_add(("Hello", ", World!")),
            "Hello, World!".to_string()
        );

        let uncurried_count_char = count_char.uncurry_once();
        assert_eq!(
            uncurried_count_char(("element", 'e')),
            count_char("element", 'e')
        );
    }
}
