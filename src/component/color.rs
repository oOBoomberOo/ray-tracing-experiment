use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub const BACKGROUND: Color = Color::new(0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    pub fn display(&self) -> [u8; 4] {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;

        [r, g, b, 0xFF]
    }
}

impl From<[f32; 3]> for Color {
    fn from(color: [f32; 3]) -> Self {
        let [r, g, b] = color;
        Color::new(r, g, b)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}
impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}
impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
impl Div for Color {
    type Output = Color;
    fn div(self, rhs: Color) -> Self::Output {
        Color::new(self.r / rhs.r, self.g / rhs.g, self.b / rhs.b)
    }
}

impl Add<f32> for Color {
    type Output = Color;
    fn add(self, rhs: f32) -> Self::Output {
        Color::new(self.r + rhs, self.g + rhs, self.b + rhs)
    }
}
impl Sub<f32> for Color {
    type Output = Color;
    fn sub(self, rhs: f32) -> Self::Output {
        Color::new(self.r - rhs, self.g - rhs, self.b - rhs)
    }
}
impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}
impl Div<f32> for Color {
    type Output = Color;
    fn div(self, rhs: f32) -> Self::Output {
        Color::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}