use crate::Float;
use core::fmt;

use super::Numeric;

#[macro_export]
macro_rules! complex {
    ($real:expr, $imaginary:expr) => {
        $crate::Complex::new($real, $imaginary)
    };
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Complex<T: Float> {
    pub real: T,
    pub imaginary: T,
}

impl<T: Float + fmt::Debug> fmt::Debug for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Complex {{ real: {:?}, imaginary: {:?} }}",
            self.real, self.imaginary
        )
    }
}

impl<T: Float + fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
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

impl<T: Float> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Complex<T> {
        Complex { real, imaginary }
    }

    pub fn get_conjugate(&self) -> Complex<T> {
        Complex {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    pub fn conjugate(&mut self) {
        self.imaginary = -self.imaginary;
    }

    pub fn phase(&self) -> T {
        T::atan2(self.imaginary, self.real)
    }

    pub fn norm(&self) -> T {
        self.real * self.real + self.imaginary * self.imaginary
    }

    pub fn abs(&self) -> T {
        T::sqrt(self.norm())
    }
}
