use super::{Float, Matrix};
use core::ops;

impl<T: Float> ops::Add<&Matrix<T>> for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: &Matrix<T>) -> Self::Output {
        self.add_to(other)
    }
}

impl<T: Float> ops::Sub<&Matrix<T>> for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: &Matrix<T>) -> Self::Output {
        self.subtract(other)
    }
}

impl<T: Float> ops::Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, scalar: T) -> Self::Output {
        self.scale(scalar)
    }
}

impl<T: Float> ops::Div<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn div(self, scalar: T) -> Self::Output {
        self.scale(T::one() / scalar)
    }
}

impl<T: Float> ops::AddAssign<&Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, other: &Matrix<T>) {
        if let Some(result) = self.add_to(other) {
            *self = result;
        }
    }
}

impl<T: Float> ops::SubAssign<&Matrix<T>> for Matrix<T> {
    fn sub_assign(&mut self, other: &Matrix<T>) {
        if let Some(result) = self.subtract(other) {
            *self = result;
        }
    }
}

impl<T: Float> ops::MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, scalar: T) {
        *self = self.scale(scalar);
    }
}

impl<T: Float> ops::DivAssign<T> for Matrix<T> {
    fn div_assign(&mut self, scalar: T) {
        *self = self.scale(T::one() / scalar);
    }
}
