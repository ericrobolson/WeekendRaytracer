pub trait NumberOps {
    fn nsin(&self) -> Self;
    fn ncos(&self) -> Self;
    fn nsqrt(&self) -> Self;
    fn fraction(numerator: i32, denominator: i32) -> Self;
    fn to_f32(&self) -> f32;

    fn from(i: i32) -> Self;
    fn pi() -> Self;
    fn infinity() -> Self;
}

pub trait Number:
    Copy
    + Clone
    + PartialEq
    + NumberOps
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::cmp::PartialOrd
where
    Self: std::marker::Sized,
{
    fn zero() -> Self;
    fn one() -> Self;
    fn nabs(&self) -> Self;
    fn nsqrd(&self) -> Self;
    fn min(a: Self, b: Self) -> Self;
    fn max(a: Self, b: Self) -> Self;
    fn as_r(&self) -> Self;
}

impl<R> Number for R
where
    R: Copy
        + Clone
        + PartialEq
        + NumberOps
        + std::ops::Add<Output = Self>
        + std::ops::Sub<Output = Self>
        + std::ops::Mul<Output = Self>
        + std::ops::Div<Output = Self>
        + std::ops::Neg<Output = Self>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::cmp::PartialOrd,
{
    fn zero() -> Self {
        Self::from(0)
    }

    fn one() -> Self {
        Self::from(1)
    }

    fn nabs(&self) -> Self {
        let v = *self;
        if v < Self::zero() {
            return -v;
        }

        v
    }

    fn nsqrd(&self) -> Self {
        *self * *self
    }

    fn min(a: Self, b: Self) -> Self {
        if a < b {
            a
        } else {
            b
        }
    }
    fn max(a: Self, b: Self) -> Self {
        if a > b {
            a
        } else {
            b
        }
    }

    fn as_r(&self) -> Self {
        *self
    }
}

impl NumberOps for f32 {
    fn ncos(&self) -> Self {
        self.cos()
    }

    fn nsin(&self) -> Self {
        self.sin()
    }

    fn nsqrt(&self) -> Self {
        self.sqrt()
    }

    fn to_f32(&self) -> f32 {
        *self
    }

    fn from(i: i32) -> Self {
        i as Self
    }

    fn fraction(numerator: i32, denominator: i32) -> Self {
        let n = numerator as Self;
        let d = denominator as Self;

        return n / d;
    }

    fn pi() -> Self {
        std::f32::consts::PI
    }

    fn infinity() -> Self {
        Self::MAX
    }
}
