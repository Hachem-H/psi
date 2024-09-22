use crate::{ColumnVector, Complex, Matrix, VectorMatrix};
use std::ops;

pub type QuantumBit = ColumnVector<Complex<f64>>;
pub type QuantumGate = Matrix<Complex<f64>>;

pub struct QuantumRegister {
    state: ColumnVector<Complex<f64>>,
    qubits: Vec<QuantumBit>,
}

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
            let mut bits: [QuantumBit; N] = [$($bit),*];
            QuantumRegister::from(&mut bits)
        }
    };
}

impl QuantumBit {
    pub fn get_state(&self) -> i32 {
        (self[1] != Complex::new(0.0, 0.0)) as i32
    }
}

impl QuantumRegister {
    pub fn from(bits: &mut [QuantumBit]) -> QuantumRegister {
        let matrices: Vec<Matrix<Complex<f64>>> = bits.iter().map(|bit| bit.to_matrix()).collect();
        let mut result = matrices[0].clone();
        for matrix in &matrices[1..] {
            result = result.kronecker(matrix);
        }

        QuantumRegister {
            qubits: bits.to_vec(),
            state: ColumnVector::from_matrix(&result),
        }
    }

    pub fn apply(&mut self, gate: &QuantumGate, index: usize) {
        let result: ColumnVector<Complex<f64>> = self.state.mul_matrix(gate).unwrap();
        self.qubits[index] = result;
    }
}

impl ops::Index<usize> for QuantumRegister {
    type Output = QuantumBit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.qubits[index]
    }
}

impl ops::IndexMut<usize> for QuantumRegister {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.qubits[index]
    }
}
