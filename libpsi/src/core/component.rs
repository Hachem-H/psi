use crate::{ColumnVector, Complex, Matrix, VectorMatrix};

pub type QuantumRegister = ColumnVector<Complex<f64>>;
pub type QuantumBit = ColumnVector<Complex<f64>>;

#[macro_export]
macro_rules! count {
    () => { 0 };
    ($head:expr $(,$tail:expr)*) => { 1 + count!($( $tail ),*) };
}

#[macro_export]
macro_rules! qubit {
    ($(($re:expr, $im:expr)),*) => {
        {
            let mut vector = Vec::new();
            $(
                vector.push(complex!($re, $im));
            )*
            QuantumBit::new(vector)
        }
    };
}

#[macro_export]
macro_rules! quantum_register {
    ($($bit:expr),*) => {
        {
            const N: usize = count!($($bit),*);
            let bits: [QuantumBit; N] = [$($bit),*];
            QuantumRegister::from(&bits)
        }
    };
}

impl QuantumBit {
    pub fn get_state(&self) -> i32 {
        (self[1] != Complex::new(0.0, 0.0)) as i32
    }
}

impl QuantumRegister {
    pub fn from(bits: &[QuantumBit]) -> QuantumRegister {
        let matrices: Vec<Matrix<Complex<f64>>> = bits.iter().map(|bit| bit.to_matrix()).collect();
        let mut result = matrices[0].clone();
        for matrix in &matrices[1..] {
            result = result.kronecker(matrix);
        }
        ColumnVector::from_matrix(&result)
    }
}
