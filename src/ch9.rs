#![allow(unused)]

fn curry<T: Clone, U, V, F: Fn(T, U) -> V + Clone>(f: F, t: T) -> impl Fn(U) -> V {
    let f = f.clone();
    move |u: U| f(t.clone(), u)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    fn test_curry_with_add() {
        let add = |x: u32, y: u32| x + y;
        let curried_add = curry(add, 1);

        assert_eq!(3, curried_add(2));
        assert_eq!(3, add(1, 2));

        let add = |x: &str, y: &str| x.to_string() + y;
        let curried_add = curry(add, "Hello");
        assert_eq!(curried_add(", World!"), add("Hello", ", World!"));

        fn count_char(s: &str, c: char) -> usize {
            s.chars().filter(|ch| ch == &c).count()
        }

        let curried_count_char = curry(count_char, "element");
        assert_eq!(curried_count_char('e'), 3usize);
        assert_eq!(count_char("element", 'e'), 3);
    }
}
