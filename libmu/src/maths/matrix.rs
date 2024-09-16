use super::Float;
use core::{fmt, ops};

pub struct Matrix<T: Float> {
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T: Float> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        Matrix { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    pub fn dot(&self, other: &Self) -> Option<Matrix<T>> {
        if self.cols != other.rows {
            return None;
        }

        let mut result = Matrix::new(
            self.rows,
            other.cols,
            vec![T::zero(); self.rows * other.cols],
        );
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum = sum + (self.get(i, k) * other.get(k, j));
                }
                result.set(i, j, sum);
            }
        }
        Some(result)
    }

    pub fn kronecker(&self, other: &Self) -> Matrix<T> {
        let new_rows = self.rows * other.rows;
        let new_cols = self.cols * other.cols;

        let mut result = Matrix::new(new_rows, new_cols, vec![T::zero(); new_rows * new_cols]);

        for i in 0..self.rows {
            for j in 0..self.cols {
                let self_val = self.get(i, j);
                for k in 0..other.rows {
                    for l in 0..other.cols {
                        let result_row = i * other.rows + k;
                        let result_col = j * other.cols + l;
                        result.set(result_row, result_col, self_val * other.get(k, l))
                    }
                }
            }
        }

        result
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut result = Matrix::new(self.cols, self.rows, vec![T::zero(); self.cols * self.rows]);

        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = self.get(i, j);
                result.set(j, i, value);
            }
        }

        result
    }

    pub fn add_to(&self, other: &Self) -> Option<Matrix<T>> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }

        let mut result = Matrix::new(self.rows, self.cols, vec![T::zero(); self.rows * self.cols]);

        for i in 0..self.rows {
            for j in 0..self.cols {
                let sum = self.get(i, j) + other.get(i, j);
                result.set(i, j, sum);
            }
        }
        Some(result)
    }

    pub fn subtract(&self, other: &Self) -> Option<Matrix<T>> {
        if self.rows != other.rows || self.cols != other.cols {
            return None;
        }

        let mut result = Matrix::new(self.rows, self.cols, vec![T::zero(); self.rows * self.cols]);

        for i in 0..self.rows {
            for j in 0..self.cols {
                let diff = self.get(i, j) - other.get(i, j);
                result.set(i, j, diff);
            }
        }
        Some(result)
    }

    pub fn scale(&self, scalar: T) -> Matrix<T> {
        let mut result = Matrix::new(self.rows, self.cols, vec![T::zero(); self.rows * self.cols]);

        for i in 0..self.rows {
            for j in 0..self.cols {
                let scaled_value = self.get(i, j) * scalar;
                result.set(i, j, scaled_value);
            }
        }
        result
    }
}

impl<T: Float> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T: Float> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}

impl<T: Float + fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Float + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:>8} ", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

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
