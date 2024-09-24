use super::{Complex, Float};
use core::ops;

impl<T: Float> ops::Neg for Complex<T> {
    type Output = Complex<T>;

    fn neg(self) -> Complex<T> {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl<T: Float> From<T> for Complex<T> {
    fn from(real: T) -> Complex<T> {
        Complex {
            real,
            imaginary: T::zero(),
        }
    }
}

// Complex-Complex
impl<T: Float> ops::Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl<T: Float> ops::Sub for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl<T: Float> ops::Mul for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }
}

impl<T: Float> ops::Div for Complex<T> {
    type Output = Complex<T>;

    fn div(self, other: Complex<T>) -> Complex<T> {
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        Complex {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denom,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denom,
        }
    }
}

// Complex-Complex, Assign
impl<T: Float> ops::AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Complex<T>) {
        self.real += other.real;
        self.imaginary += other.imaginary;
    }
}

impl<T: Float> ops::SubAssign for Complex<T> {
    fn sub_assign(&mut self, other: Complex<T>) {
        self.real -= other.real;
        self.imaginary -= other.imaginary;
    }
}

impl<T: Float> ops::MulAssign for Complex<T> {
    fn mul_assign(&mut self, other: Complex<T>) {
        let new_real = self.real * other.real - self.imaginary * other.imaginary;
        let new_imaginary = self.real * other.imaginary + self.imaginary * other.real;
        self.real = new_real;
        self.imaginary = new_imaginary;
    }
}

impl<T: Float> ops::DivAssign for Complex<T> {
    fn div_assign(&mut self, other: Complex<T>) {
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        let new_real = (self.real * other.real + self.imaginary * other.imaginary) / denom;
        let new_imaginary = (self.imaginary * other.real - self.real * other.imaginary) / denom;
        self.real = new_real;
        self.imaginary = new_imaginary;
    }
}

// Real-Complex
impl<T: Float> ops::Add<T> for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: T) -> Complex<T> {
        Complex {
            real: self.real + other,
            imaginary: self.imaginary,
        }
    }
}

impl<T: Float> ops::Sub<T> for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, other: T) -> Complex<T> {
        Complex {
            real: self.real - other,
            imaginary: self.imaginary,
        }
    }
}

impl<T: Float> ops::Mul<T> for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: T) -> Complex<T> {
        Complex {
            real: self.real * other,
            imaginary: self.imaginary * other,
        }
    }
}

impl<T: Float> ops::Div<T> for Complex<T> {
    type Output = Complex<T>;

    fn div(self, other: T) -> Complex<T> {
        Complex {
            real: self.real / other,
            imaginary: self.imaginary / other,
        }
    }
}
