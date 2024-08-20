// <https://bartoszmilewski.com/2014/11/04/category-the-essence-of-composition/>
// (1) implement the id func
// (2) implement func composition
// (3) verify (2) respects (1)

#![allow(dead_code)]

// (1): id function
pub fn id<T>(x: T) -> T {
    x
}

// (2) composition function
pub fn compose<F, G, T, U, V>(f: F, g: G) -> impl Fn(T) -> V
where
    F: Fn(T) -> U,
    G: Fn(U) -> V,
{
    move |arg| g(f(arg))
}

// extracting out compositions would make for greater clarity
#[cfg(test)]
mod tests {
    use super::*;

    fn add_one(x: u32) -> u32 {
        x + 1
    }

    fn double(x: u32) -> u32 {
        x * 2
    }

    #[test]
    fn compose_add_one_and_double() {
        let x = double(add_one(1));
        let y = (compose(add_one, double))(1);
        assert_eq!(x, y);
    }

    // (3) test id . id = id
    #[test]
    fn test_compose_id_with_itself() {
        let x = "id";
        assert_eq!(id(x), (compose(id, id))(x));
    }

    #[test]
    fn test_id_composed_with_double_is_double() {
        let x = double(id(42));
        let double_dot_id = compose(double, id);
        assert_eq!(x, double_dot_id(42));
    }

    #[test]
    fn test_double_composed_with_id_is_double() {
        let x = id(double(42));
        let id_dot_double = compose(id, double);
        assert_eq!(x, id_dot_double(42));
    }
}
