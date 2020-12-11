use crate::math::{Vec3, R};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: R,
    pub g: R,
    pub b: R,
    pub a: R,
}

fn u8_r(n: u8) -> R {
    (n as R) / 255.
}

fn r_u8(n: R) -> u8 {
    (n * 255.) as u8
}

impl Color {
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: u8_r(r),
            g: u8_r(g),
            b: u8_r(b),
            a: u8_r(a),
        }
    }

    pub fn new(r: R, g: R, b: R, a: R) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_vec3(v: Vec3, alpha: u8) -> Self {
        Self {
            r: v.x,
            g: v.y,
            b: v.z,
            a: u8_r(alpha),
        }
    }

    pub fn from_samples(&self, samples: u32) -> Self {
        *self / samples
    }
}

impl std::ops::Div<u32> for Color {
    type Output = Color;
    fn div(self, rhs: u32) -> Color {
        let rhs = rhs as R;
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [r_u8(self.r), r_u8(self.g), r_u8(self.b), r_u8(self.a)]
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}
