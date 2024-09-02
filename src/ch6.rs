// <https://bartoszmilewski.com/2015/01/13/simple-algebraic-data-types/>
// Challenges (2), (3) and (4): impl a Shape sum type w/ 3 variants (Circle,
// Rectangle, Square) and two methods to compute their area and perimeter.

#![allow(unused)]

enum Shape {
    Circle { r: f64 },
    Rectangle { d: f64, h: f64 },
    Square { s: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle { r } => std::f64::consts::PI * r * r,
            Self::Rectangle { d, h } => d * h,
            Self::Square { s } => s * s,
        }
    }

    fn perim(&self) -> f64 {
        match self {
            Self::Circle { r } => 2.0 * std::f64::consts::PI * r,
            Self::Rectangle { d, h } => 2.0 * (d + h),
            Self::Square { s } => 4.0 * s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[track_caller]
    fn test_area() {
        let pi = std::f64::consts::PI;
        let x = 2.0;
        let y = 3.0;

        let circle = Shape::Circle { r: x };
        let rect = Shape::Rectangle { d: x, h: y };
        let square = Shape::Square { s: x };

        assert_eq!(circle.area(), pi * x * x);
        assert_eq!(rect.area(), x * y);
        assert_eq!(square.area(), x * x);
    }

    #[test]
    #[track_caller]
    fn test_perim() {
        let pi = std::f64::consts::PI;
        let x = 2.0;
        let y = 3.0;

        let circle = Shape::Circle { r: x };
        let rect = Shape::Rectangle { d: x, h: y };
        let square = Shape::Square { s: x };

        assert_eq!(circle.perim(), 2.0 * pi * x);
        assert_eq!(rect.perim(), 2.0 * (x + y));
        assert_eq!(square.perim(), 4.0 * x);
    }
}
