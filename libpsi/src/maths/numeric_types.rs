use super::Complex;
use core::ops;

pub trait Numeric:
    Copy
    + PartialOrd
    + ops::Add<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Div<Output = Self>
    + ops::Neg<Output = Self>
    + ops::AddAssign
    + ops::SubAssign
    + ops::MulAssign
    + ops::DivAssign
{
    fn zero() -> Self;
    fn one() -> Self;
}

pub trait Integer: Numeric {}
pub trait Float: Numeric {
    fn sqrt(self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    fn atan2(y: Self, x: Self) -> Self {
        libm::atan2f(y, x)
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    fn atan2(y: Self, x: Self) -> Self {
        libm::atan2(y, x)
    }
}

impl Float for Complex<f32> {
    fn sqrt(self) -> Self {
        let r = self.abs();
        let theta = self.phase();

        let sqrt_r = libm::sqrtf(r);
        let sqrt_theta = theta / 2.0;

        Complex::new(
            sqrt_r * libm::cosf(sqrt_theta),
            sqrt_r * libm::sinf(sqrt_theta),
        )
    }

    fn atan2(y: Self, x: Self) -> Self {
        Complex::new(
            libm::atan2f(y.real, x.real),
            libm::atan2f(y.imaginary, x.imaginary),
        )
    }
}

impl Float for Complex<f64> {
    fn sqrt(self) -> Self {
        let r = self.abs();
        let theta = self.phase();

        let sqrt_r = libm::sqrt(r);
        let sqrt_theta = theta / 2.0;

        Complex::new(
            sqrt_r * libm::cos(sqrt_theta),
            sqrt_r * libm::sin(sqrt_theta),
        )
    }

    fn atan2(y: Self, x: Self) -> Self {
        Complex::new(
            libm::atan2(y.real, x.real),
            libm::atan2(y.imaginary, x.imaginary),
        )
    }
}

impl Integer for i64 {}
impl Integer for i32 {}

impl Numeric for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Numeric for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Numeric for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl Numeric for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

impl Numeric for Complex<f32> {
    fn zero() -> Self {
        Complex::new(0.0, 0.0)
    }

    fn one() -> Self {
        Complex::new(1.0, 0.0)
    }
}

impl Numeric for Complex<f64> {
    fn zero() -> Self {
        Complex::new(0.0, 0.0)
    }

    fn one() -> Self {
        Complex::new(1.0, 0.0)
    }
}
