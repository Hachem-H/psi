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
