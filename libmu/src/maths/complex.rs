use crate::numeric_types::FloatTypes;
use std::ops;

pub struct Complex<T: FloatTypes> {
    pub real: T,
    pub imaginary: T,
}

impl<T: FloatTypes> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Complex<T> {
        Complex { real, imaginary }
    }

    pub fn get_conjugate(&self) -> Complex<T> {
        Complex::new(self.real, -self.imaginary)
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

impl<T: FloatTypes> ops::Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl<T: FloatTypes> ops::Sub for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl<T: FloatTypes> ops::Mul for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl<T: FloatTypes> ops::Div for Complex<T> {
    type Output = Complex<T>;

    fn div(self, other: Complex<T>) -> Complex<T> {
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        Complex {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denom,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denom,
        }
    }
}

impl<T: FloatTypes> ops::Add<T> for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: T) -> Complex<T> {
        Complex {
            real: self.real + other,
            imaginary: self.imaginary,
        }
    }
}

impl<T: FloatTypes> ops::Sub<T> for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, other: T) -> Complex<T> {
        Complex {
            real: self.real - other,
            imaginary: self.imaginary,
        }
    }
}

impl<T: FloatTypes> ops::Mul<T> for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: T) -> Complex<T> {
        Complex {
            real: self.real * other,
            imaginary: self.imaginary * other,
        }
    }
}

impl<T: FloatTypes> ops::Div<T> for Complex<T> {
    type Output = Complex<T>;

    fn div(self, other: T) -> Complex<T> {
        Complex {
            real: self.real / other,
            imaginary: self.imaginary / other,
        }
    }
}

impl<T: FloatTypes> ops::AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Complex<T>) {
        self.real += other.real;
        self.imaginary += other.imaginary;
    }
}

impl<T: FloatTypes> ops::SubAssign for Complex<T> {
    fn sub_assign(&mut self, other: Complex<T>) {
        self.real -= other.real;
        self.imaginary -= other.imaginary;
    }
}

impl<T: FloatTypes> ops::MulAssign for Complex<T> {
    fn mul_assign(&mut self, other: Complex<T>) {
        let new_real = self.real * other.real - self.imaginary * other.imaginary;
        let new_imaginary = self.real * other.imaginary + self.imaginary * other.real;
        self.real = new_real;
        self.imaginary = new_imaginary;
    }
}

impl<T: FloatTypes> ops::DivAssign for Complex<T> {
    fn div_assign(&mut self, other: Complex<T>) {
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        let new_real = (self.real * other.real + self.imaginary * other.imaginary) / denom;
        let new_imaginary = (self.imaginary * other.real - self.real * other.imaginary) / denom;
        self.real = new_real;
        self.imaginary = new_imaginary;
    }
}

impl<T: FloatTypes> ops::AddAssign<T> for Complex<T> {
    fn add_assign(&mut self, other: T) {
        self.real += other;
    }
}

impl<T: FloatTypes> ops::SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self, other: T) {
        self.real -= other;
    }
}

impl<T: FloatTypes> ops::MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self, other: T) {
        self.real *= other;
        self.imaginary *= other;
    }
}

impl<T: FloatTypes> ops::DivAssign<T> for Complex<T> {
    fn div_assign(&mut self, other: T) {
        self.real /= other;
        self.imaginary /= other;
    }
}
