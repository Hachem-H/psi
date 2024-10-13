use crate::{complex, ColumnVector, Complex, Matrix, Vector, VectorMatrix};
use core::{fmt, ops};

pub type QuantumState = ColumnVector<Complex<f64>>;
pub type QuantumBit = QuantumState;

#[derive(Clone)]
pub struct QuantumGate {
    pub name: &'static str,
    pub matrix: Matrix<Complex<f64>>,
}

#[derive(Clone)]
pub struct ClassicalRegister {
    bits: Vec<i32>,
}

#[derive(Clone)]
pub struct QuantumRegister {
    state_vector: QuantumState,
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
    pub fn state_0() -> QuantumBit {
        QuantumBit::new(vec![complex!(1.0, 0.0), complex!(0.0, 0.0)])
    }

    pub fn state_1() -> QuantumBit {
        QuantumBit::new(vec![complex!(0.0, 0.0), complex!(1.0, 0.0)])
    }
}

impl fmt::Display for QuantumGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

        self.state_vector = ColumnVector::from_matrix(&new_result);
    }

    pub fn new(count: usize) -> QuantumRegister {
        QuantumRegister::from(&mut vec![QuantumBit::state_0(); count])
    }

    pub fn from(bits: &mut [QuantumBit]) -> QuantumRegister {
        let mut register = QuantumRegister {
            qubits: bits.to_vec(),
            state_vector: ColumnVector::new(vec![]),
        };

        register.update();
        register
    }

    pub fn get_bits(&self) -> Vec<QuantumBit> {
        self.qubits.clone()
    }

    pub fn get_state(&self) -> QuantumState {
        self.state_vector.clone()
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
