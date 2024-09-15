use std::ops;

pub trait NumericTypes:
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
}

pub trait IntTypes: NumericTypes {}
pub trait FloatTypes: NumericTypes {
    fn sqrt(self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;
}

impl FloatTypes for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn atan2(y: Self, x: Self) -> Self {
        y.atan2(x)
    }
}

impl FloatTypes for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn atan2(y: Self, x: Self) -> Self {
        y.atan2(x)
    }
}

impl NumericTypes for i32 {}
impl NumericTypes for i64 {}
impl NumericTypes for f32 {}
impl NumericTypes for f64 {}
