#![allow(dead_code)]

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {

        Color { 
            r, 
            g, 
            b, 
            a
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: Color) -> Color {
        Color { 
            r: self.r + other.r, 
            g: self.g + other.g, 
            b: self.b + other.b, 
            a: self.a + other.a 
        }
    }
}

impl ops::Add<f32> for Color {
    type Output = Color;

    #[inline]
    fn add(self, other: f32) -> Color {
        Color { 
            r: self.r + other, 
            g: self.g + other, 
            b: self.b + other, 
            a: self.a + other 
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    #[inline]
    fn sub(self, other: Color) -> Color {
        Color { 
            r: self.r - &other.r, 
            g: self.g - &other.g, 
            b: self.b - &other.b, 
            a: self.a - &other.a
        }
    }
}

impl ops::Sub<f32> for Color {
    type Output = Color;

    #[inline]
    fn sub(self, other: f32) -> Color {
        Color { 
            r: self.r - other, 
            g: self.g - other, 
            b: self.b - other, 
            a: self.a - other 
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, other: Color) -> Color {
        Color { 
            r: self.r * &other.r, 
            g: self.g * &other.g,
            b: self.b * &other.b, 
            a: self.a * &other.a 
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, other: f32) -> Color {
        Color { 
            r: self.r * other, 
            g: self.g * other, 
            b: self.b * other, 
            a: self.a * other 
        }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    #[inline]
    fn mul(self, other: Color) -> Color {
        Color { 
            r: self * &other.r, 
            g: self * &other.g, 
            b: self * &other.b, 
            a: self * &other.a 
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

