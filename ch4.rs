// <https://bartoszmilewski.com/2014/12/23/kleisli-categories/>
// 1. Kleisli cat for partial funcs
// 2. impl safe_reciprocal()
// 3. compose safe_root() and safe_reciprocal()
// the writer mod done as an extra exercise that sticks to the matching
// Haskell code in the post - it's neither v. idiomatic nor efficient Rust.

#![allow(unused)]

pub mod optional {
    fn identity_morphism<T>(v: T) -> Option<T> {
        Some(v)
    }

    fn compose<T, U>(
        f: impl Fn(T) -> Option<U>,
        g: impl Fn(U) -> Option<U>,
    ) -> impl Fn(T) -> Option<U> {
        // Clippy recommends passing `g` to `and_then` directly,
        // but this triggers <https://doc.rust-lang.org/error_codes/E0507.html>
        move |x| f(x).and_then(|y| g(y))
    }

    fn safe_root(arg: f64) -> Option<f64> {
        match arg {
            _ if arg >= 0.0 => Some(arg.sqrt()),
            _ => None,
        }
    }

    fn safe_reciprocal(arg: f64) -> Option<f64> {
        match arg {
            _ if arg != 0.0 => Some(1.0 / arg),
            _ => None,
        }
    }

    pub fn safe_root_reciprocal(arg: f64) -> Option<f64> {
        (compose(safe_reciprocal, safe_root))(arg)
    }
}

pub mod writer {
    #[derive(Debug, PartialEq)]
    pub struct Writer<T>(pub T, pub String);

    pub fn identity_morphism<T>(v: T) -> Writer<T> {
        Writer(v, "".to_string())
    }

    pub fn compose<T, U, V>(
        f: impl Fn(T) -> Writer<U>,
        g: impl Fn(U) -> Writer<V>,
    ) -> impl Fn(T) -> Writer<V> {
        move |x| {
            let Writer(y, s1) = f(x);
            let Writer(z, s2) = g(y);
            Writer(z, s1 + &s2)
        }
    }

    pub fn upcase(s: String) -> Writer<Vec<char>> {
        Writer(
            s.chars()
                .map(|c| c.to_ascii_uppercase())
                .collect::<Vec<char>>(),
            "upcase ".to_string(),
        )
    }

    pub fn to_words(chars: Vec<char>) -> Writer<Vec<String>> {
        Writer(
            chars
                .into_iter()
                .collect::<String>()
                .split_ascii_whitespace()
                .map(|word| word.to_string())
                .collect(),
            "to_words ".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::optional::*;
    use super::writer::*;

    #[test]
    fn test_upcase_to_words() {
        let upcase_to_words = compose(upcase, to_words);
        let s = "And now for something completely the same".to_owned();
        let words = s
            .split_ascii_whitespace()
            .map(|s| s.to_ascii_uppercase().to_owned())
            .collect();
        let log = "upcase to_words ".to_owned();
        assert_eq!(upcase_to_words(s), Writer(words, log));
    }

    #[test]
    fn test_safe_root_reciprocal_zero() {
        assert_eq!(safe_root_reciprocal(0.0), None);
    }

    #[test]
    fn test_safe_root_reciprocal_negative_number() {
        assert_eq!(safe_root_reciprocal(-4.0), None);
    }

    #[test]
    fn test_safe_root_reciprocal_positive_number() {
        assert_eq!(safe_root_reciprocal(4.0), Some(0.5));
    }
}
