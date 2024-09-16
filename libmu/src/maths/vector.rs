use super::{Float, Matrix};
use core::ops;

#[macro_export]
macro_rules! row_vector {
    ($($x:expr),*) => {
        RowVector::new(vec![$($x),*])
    };
    ($($x:expr,)*) => {
        RowVector::new(vec![$($x),*])
    };
}

#[macro_export]
macro_rules! column_vector {
    ($($x:expr),*) => {
        ColumnVector::new(vec![$($x),*])
    };
    ($($x:expr,)*) => {
        ColumnVector::new(vec![$($x),*])
    };
}

pub trait Vector<T: Float> {
    fn new(data: Vec<T>) -> Self;
    fn get(&self, index: usize) -> T;
    fn set(&mut self, index: usize, value: T);
    fn size(&self) -> usize;

    fn dot(&self, other: &Self) -> T;
    fn norm(&self) -> T;

    fn max(&self) -> T;
    fn min(&self) -> T;
    fn sum(&self) -> T;
}

pub trait VectorMatrix<T: Float> {
    fn from_matrix(matrix: &Matrix<T>) -> Self;
    fn to_matrix(&self) -> Matrix<T>;
}

pub struct VectorImpl<T: Float, const ROWS: usize, const COLS: usize>(Vec<T>);
pub type RowVector<T> = VectorImpl<T, 1, 0>;
pub type ColumnVector<T> = VectorImpl<T, 0, 1>;

impl<T: Float> ColumnVector<T> {
    pub fn mul_matrix(&self, matrix: &Matrix<T>) -> Option<ColumnVector<T>> {
        if matrix.cols != self.size() {
            return None;
        }

        let mut result = ColumnVector::new(vec![T::zero(); matrix.rows]);

        for i in 0..matrix.rows {
            let mut sum = T::zero();
            for j in 0..matrix.cols {
                sum = sum + (matrix.get(i, j) * self.get(j));
            }
            result.set(i, sum);
        }

        Some(result)
    }

    pub fn transpose(&self) -> RowVector<T> {
        RowVector::new(self.0.clone())
    }
}

impl<T: Float> RowVector<T> {
    pub fn mul_matrix(&self, matrix: &Matrix<T>) -> Option<RowVector<T>> {
        if self.size() != matrix.rows {
            return None;
        }

        let mut result = RowVector::new(vec![T::zero(); matrix.cols]);

        for j in 0..matrix.cols {
            let mut sum = T::zero();
            for i in 0..matrix.rows {
                sum = sum + (self.get(i) * matrix.get(i, j));
            }
            result.set(j, sum);
        }

        Some(result)
    }

    pub fn transpose(&self) -> ColumnVector<T> {
        ColumnVector::new(self.0.clone())
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> Vector<T> for VectorImpl<T, ROWS, COLS> {
    fn new(data: Vec<T>) -> Self {
        Self(data)
    }

    fn get(&self, index: usize) -> T {
        self.0[index]
    }

    fn set(&mut self, index: usize, value: T) {
        self.0[index] = value;
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn dot(&self, other: &Self) -> T {
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a * *b)
            .fold(T::zero(), |acc, x| acc + x)
    }

    fn norm(&self) -> T {
        self.0
            .iter()
            .map(|x| *x * *x)
            .fold(T::zero(), |acc, x| acc + x)
            .sqrt()
    }

    fn max(&self) -> T {
        *self
            .0
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&T::zero())
    }

    fn min(&self) -> T {
        *self
            .0
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&T::zero())
    }

    fn sum(&self) -> T {
        self.0.iter().fold(T::zero(), |acc, x| acc + *x)
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> VectorImpl<T, ROWS, COLS> {
    pub fn add_to(&self, other: &Self) -> Option<VectorImpl<T, ROWS, COLS>> {
        if self.size() != other.size() {
            return None;
        }

        let mut result = VectorImpl::new(vec![T::zero(); ROWS * COLS]);

        for i in 0..self.size() {
            let sum = self.get(i) + other.get(i);
            result.set(i, sum);
        }

        Some(result)
    }

    pub fn subtract(&self, other: &Self) -> Option<VectorImpl<T, ROWS, COLS>> {
        if self.size() != other.size() {
            return None;
        }

        let mut result = VectorImpl::new(vec![T::zero(); ROWS * COLS]);

        for i in 0..self.size() {
            let sum = self.get(i) - other.get(i);
            result.set(i, sum);
        }

        Some(result)
    }

    pub fn scale(&self, scalar: T) -> VectorImpl<T, ROWS, COLS> {
        let mut result = VectorImpl::new(vec![T::zero(); ROWS * COLS]);

        for i in 0..self.size() {
            let product = self.get(i) * scalar;
            result.set(i, product);
        }

        result
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::Index<usize>
    for VectorImpl<T, ROWS, COLS>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::IndexMut<usize>
    for VectorImpl<T, ROWS, COLS>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::Add<&VectorImpl<T, ROWS, COLS>>
    for VectorImpl<T, ROWS, COLS>
{
    type Output = Option<VectorImpl<T, ROWS, COLS>>;

    fn add(self, other: &VectorImpl<T, ROWS, COLS>) -> Self::Output {
        self.add_to(other)
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::Sub<&VectorImpl<T, ROWS, COLS>>
    for VectorImpl<T, ROWS, COLS>
{
    type Output = Option<VectorImpl<T, ROWS, COLS>>;

    fn sub(self, other: &VectorImpl<T, ROWS, COLS>) -> Self::Output {
        self.subtract(other)
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::Mul<T> for VectorImpl<T, ROWS, COLS> {
    type Output = VectorImpl<T, ROWS, COLS>;

    fn mul(self, scalar: T) -> Self::Output {
        self.scale(scalar)
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::Div<T> for VectorImpl<T, ROWS, COLS> {
    type Output = VectorImpl<T, ROWS, COLS>;

    fn div(self, scalar: T) -> Self::Output {
        self.scale(T::one() / scalar)
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::AddAssign<VectorImpl<T, ROWS, COLS>>
    for VectorImpl<T, ROWS, COLS>
{
    fn add_assign(&mut self, other: VectorImpl<T, ROWS, COLS>) {
        if let Some(result) = self.add_to(&other) {
            *self = result;
        }
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::SubAssign<VectorImpl<T, ROWS, COLS>>
    for VectorImpl<T, ROWS, COLS>
{
    fn sub_assign(&mut self, other: VectorImpl<T, ROWS, COLS>) {
        if let Some(result) = self.subtract(&other) {
            *self = result;
        }
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::MulAssign<T>
    for VectorImpl<T, ROWS, COLS>
{
    fn mul_assign(&mut self, scalar: T) {
        *self = self.scale(scalar);
    }
}

impl<T: Float, const ROWS: usize, const COLS: usize> ops::DivAssign<T>
    for VectorImpl<T, ROWS, COLS>
{
    fn div_assign(&mut self, scalar: T) {
        *self = self.scale(T::one() / scalar);
    }
}

impl<T: Float> VectorMatrix<T> for RowVector<T> {
    fn to_matrix(&self) -> Matrix<T> {
        Matrix::new(1, self.size(), self.0.clone())
    }

    fn from_matrix(matrix: &Matrix<T>) -> Self {
        Self::new(matrix.data.clone())
    }
}

impl<T: Float> VectorMatrix<T> for ColumnVector<T> {
    fn to_matrix(&self) -> Matrix<T> {
        Matrix::new(self.size(), 1, self.0.clone())
    }

    fn from_matrix(matrix: &Matrix<T>) -> Self {
        Self::new(matrix.data.clone())
    }
}

impl<T: Float> ops::Mul<&Matrix<T>> for RowVector<T> {
    type Output = Option<RowVector<T>>;

    fn mul(self, matrix: &Matrix<T>) -> Self::Output {
        self.mul_matrix(matrix)
    }
}

impl<T: Float> ops::Mul<&Matrix<T>> for ColumnVector<T> {
    type Output = Option<ColumnVector<T>>;

    fn mul(self, matrix: &Matrix<T>) -> Self::Output {
        self.mul_matrix(matrix)
    }
}

impl<T: Float> ops::MulAssign<&Matrix<T>> for RowVector<T> {
    fn mul_assign(&mut self, matrix: &Matrix<T>) {
        if let Some(result) = self.mul_matrix(matrix) {
            *self = result;
        }
    }
}

impl<T: Float> ops::MulAssign<&Matrix<T>> for ColumnVector<T> {
    fn mul_assign(&mut self, matrix: &Matrix<T>) {
        if let Some(result) = self.mul_matrix(matrix) {
            *self = result;
        }
    }
}
