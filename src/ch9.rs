#![allow(unused)]

fn curry<T: Clone, U, V, F: Fn(T, U) -> V + Clone>(f: F, t: T) -> impl Fn(U) -> V {
    let f = f.clone();
    move |u: U| f(t.clone(), u)
}

fn uncurry<T: Clone, U: Clone, V, F: Fn(T, U) -> V + Clone>(f: F) -> impl Fn((T, U)) -> V {
    let f = f.clone();
    move |(t, u)| f(t, u)
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
        let curried_add = curry(add, 1);
        assert_eq!(curried_add(2), add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let curried_add = curry(add, "Hello");
        assert_eq!(curried_add(", World!"), add("Hello", ", World!"));

        let curried_count_char = curry(count_char, "element");
        assert_eq!(curried_count_char('e'), count_char("element", 'e'));
    }

    #[test]
    fn test_uncurry() {
        let add = |x: u32, y: u32| x + y;
        let uncurried_add = uncurry(add);
        assert_eq!(uncurried_add((1, 2)), add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let uncurried_add = uncurry(add);
        assert_eq!(
            uncurried_add(("Hello", ", World!")),
            "Hello, World!".to_string()
        );

        let uncurried_count_char = uncurry(count_char);
        assert_eq!(
            uncurried_count_char(("element", 'e')),
            count_char("element", 'e')
        );
    }
}
