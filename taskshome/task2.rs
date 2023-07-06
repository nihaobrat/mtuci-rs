fn main() {}

struct Rect {
    top_left: (f32, f32),
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Rect { top_left, width }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let (x, y) = self.top_left;
        (x + self.width, y - self.width)
    }

    fn area(&self) -> f32 {
        self.width * self.width
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.width
    }
}

// ----> TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);
        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);
        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);
        assert_eq!(20., rect.perimeter())
    }
}
