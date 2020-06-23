#![allow(dead_code)]

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

fn clamp (n: f32) -> f32 {
    //if n < 0. { return 0. }
    //else if n > 1. { return 1. }
    //else { return n };
    return n;
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {

        Color { 
            r: clamp(r), 
            g: clamp(g), 
            b: clamp(b), 
            a: clamp(a) 
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: Color) -> Color {
        Color { 
            r: clamp(&self.r + &other.r), 
            g: clamp(&self.g + &other.g), 
            b: clamp(&self.b + &other.b), 
            a: clamp(&self.a + &other.a) 
        }
    }
}

impl ops::Add<f32> for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: f32) -> Color {
        Color { 
            r: clamp(&self.r + other), 
            g: clamp(&self.g + other), 
            b: clamp(&self.b + other), 
            a: clamp(&self.a + other) 
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    #[inline]
    fn sub(self, other: Color) -> Color {
        Color { 
            r: clamp(&self.r - &other.r), 
            g: clamp(&self.g - &other.g), 
            b: clamp(&self.b - &other.b), 
            a: clamp(&self.a - &other.a)
        }
    }
}

impl ops::Sub<f32> for Color {
    type Output = Color;

    #[inline]
    fn sub(self, other: f32) -> Color {
        Color { 
            r: clamp(&self.r - other), 
            g: clamp(&self.g - other), 
            b: clamp(&self.b - other), 
            a: clamp(&self.a - other) 
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, other: Color) -> Color {
        Color { 
            r: clamp(&self.r * &other.r), 
            g: clamp(&self.g * &other.g),
            b: clamp(&self.b * &other.b), 
            a: clamp(&self.a * &other.a) 
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, other: f32) -> Color {
        Color { 
            r: clamp(&self.r * other), 
            g: clamp(&self.g * other), 
            b: clamp(&self.b * other), 
            a: clamp(&self.a * other) 
        }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    #[inline]
    fn mul(self, other: Color) -> Color {
        Color { 
            r: clamp(self * &other.r), 
            g: clamp(self * &other.g), 
            b: clamp(self * &other.b), 
            a: clamp(self * &other.a) 
        }
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;

    #[inline]
    fn div(self, other: f32) -> Color {
        return (1. / other) * self;
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