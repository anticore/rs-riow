#![allow(dead_code)]

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color {
        let mut _r = r;
        let mut _g = g;
        let mut _b = b;
        let mut _a = a;

        if _r < 0. { _r = 0.; }
        else if _r > 1. { _r = 1. }
        if _g < 0. { _g = 0.; }
        else if _g > 1. { _g = 1. }
        if _b < 0. { _b = 0.; }
        else if _b > 1. { _b = 1. }

        Color { r:_r, g:_g, b:_b, a: _a }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_new() {
        let color: Color = Color::new(0., 0., 0., 1.);
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);
        assert_eq!(color.a, 1.);

        let color = Color::new(-1., 0.4, 2., 1.);
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.4);
        assert_eq!(color.b, 1.);
    }
}