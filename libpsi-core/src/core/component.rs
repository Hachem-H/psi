use rand::Rng;

use crate::{complex, ColumnVector, Complex, Matrix, Vector, VectorMatrix};
use core::ops;

pub type QuantumBit = ColumnVector<Complex<f64>>;
pub type QuantumGate = Matrix<Complex<f64>>;

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

#[derive(Clone)]
pub struct ClassicalRegister {
    bits: Vec<i32>,
}

pub struct QuantumRegister {
    state: ColumnVector<Complex<f64>>,
    qubits: Vec<QuantumBit>,
}

impl QuantumBit {
    pub fn measure(&self) -> i32 {
        let alpha_abs = self[0].abs();
        let beta_abs = self[1].abs();

        let alpha_norm = alpha_abs * alpha_abs;
        let beta_norm = beta_abs * beta_abs;

        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..(alpha_norm as f32 + beta_norm as f32));

        if random_value < alpha_norm as f32 {
            0
        } else {
            1
        }
    }

    pub fn state_0() -> QuantumBit {
        QuantumBit::new(vec![complex!(1.0, 0.0), complex!(0.0, 0.0)])
    }

    pub fn state_1() -> QuantumBit {
        QuantumBit::new(vec![complex!(0.0, 0.0), complex!(1.0, 0.0)])
    }
}

impl ClassicalRegister {
    pub fn new(count: usize) -> ClassicalRegister {
        ClassicalRegister {
            bits: Vec::with_capacity(count),
        }
    }

    pub fn set_bits(&mut self, bits: Vec<i32>) {
        self.bits = bits;
    }

    pub fn get_bits(&self) -> Vec<i32> {
        self.bits.clone()
    }
}

impl QuantumRegister {
    fn update(&mut self) {
        let matrices: Vec<Matrix<Complex<f64>>> =
            self.qubits.iter().map(|qubit| qubit.to_matrix()).collect();
        let mut new_result = matrices[0].clone();
        for matrix in &matrices[1..] {
            new_result = new_result.kronecker(matrix);
        }

        self.state = ColumnVector::from_matrix(&new_result);
    }

    pub fn from(bits: &mut [QuantumBit]) -> QuantumRegister {
        let mut register = QuantumRegister {
            qubits: bits.to_vec(),
            state: ColumnVector::new(vec![]),
        };

        register.update();
        register
    }

    pub fn measure(&self, classical_register: &mut ClassicalRegister) {
        classical_register.set_bits(self.qubits.iter().map(|qubit| qubit.measure()).collect());
    }

    pub fn get_bits(&self) -> Vec<QuantumBit> {
        self.qubits.clone()
    }

    pub fn get_state(&self) -> ColumnVector<Complex<f64>> {
        self.state.clone()
    }

    pub fn apply(&self, gate: &QuantumGate) -> QuantumBit {
        self.state.mul_matrix(gate).unwrap()
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

impl ops::Index<usize> for ClassicalRegister {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl ops::IndexMut<usize> for ClassicalRegister {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bits[index]
    }
}
