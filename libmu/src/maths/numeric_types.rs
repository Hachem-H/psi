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
