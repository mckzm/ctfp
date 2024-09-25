// <https://bartoszmilewski.com/2015/04/07/natural-transformations/>

#![allow(unused)]

// safeHead Natural transformation: List -> Maybe functor (&[T] -> Option<T>)
fn safe_head<T: Clone>(xs: &[T]) -> Option<T> {
    xs.first().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_head_mt() {
        let xs: [u32; 0] = [];
        assert_eq!(safe_head(&xs), None);
    }

    #[test]
    fn test_safe_head_singleton() {
        let xs = [42];
        assert_eq!(safe_head(&xs), Some(42));
    }

    #[test]
    fn test_safe_head_regular_list() {
        let xs = Vec::from_iter(0..=10);
        assert_eq!(safe_head(&xs), Some(0));
    }
}
