use crate::{column_vector, complex, ColumnVector, Complex, Matrix, Vector, VectorMatrix};
use core::{fmt, ops};

// TODO(Hachem): Redo these macros to work with the new function definition.
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

pub type QuantumState = ColumnVector<Complex<f64>>;
impl QuantumState {
    pub fn state_0() -> QuantumState {
        column_vector![complex!(1.0, 0.0), complex!(0.0, 0.0)]
    }

    pub fn state_1() -> QuantumState {
        column_vector![complex!(0.0, 0.0), complex!(1.0, 0.0)]
    }
}

#[derive(Clone)]
pub struct QuantumBit<'a> {
    state: QuantumState,
    name: &'a str,
}

#[derive(Clone)]
pub struct QuantumRegister<'a> {
    state_vector: QuantumState,
    name: &'a str,
    qubits: Vec<QuantumBit<'a>>,
}

#[derive(Clone)]
pub struct QuantumGate<'a> {
    pub name: &'a str,
    pub matrix: Matrix<Complex<f64>>,
}

impl<'a> QuantumBit<'a> {
    pub fn new(name: &'a str, state: QuantumState) -> QuantumBit<'a> {
        QuantumBit { name, state }
    }

    pub fn get_state(&self) -> QuantumState {
        self.state.clone()
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a> QuantumRegister<'a> {
    pub fn new(name: &'a str, names: &[&'a str]) -> QuantumRegister<'a> {
        let mut bits: Vec<QuantumBit<'a>> = Vec::new();
        for i in 0..names.len() {
            bits.push(QuantumBit::new(names[i], QuantumState::state_0()))
        }

        QuantumRegister::from(name, &mut bits)
    }

    pub fn from(name: &'a str, bits: &mut [QuantumBit<'a>]) -> QuantumRegister<'a> {
        let mut register = QuantumRegister {
            name,
            qubits: bits.to_vec(),
            state_vector: ColumnVector::new(vec![]),
        };

        register.update();
        register
    }

    fn update(&mut self) {
        let matrices: Vec<Matrix<Complex<f64>>> = self
            .qubits
            .iter()
            .map(|qubit| qubit.state.to_matrix())
            .collect();
        let mut new_result = matrices[0].clone();
        for matrix in &matrices[1..] {
            new_result = new_result.kronecker(matrix);
        }

        self.state_vector = ColumnVector::from_matrix(&new_result);
    }

    pub fn get_bits(&self) -> Vec<QuantumBit> {
        self.qubits.clone()
    }

    pub fn get_state(&self) -> QuantumState {
        self.state_vector.clone()
    }

    pub fn get_name(&self) -> &'a str {
        self.name
    }
}

impl<'a> ops::Index<usize> for QuantumRegister<'a> {
    type Output = QuantumBit<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.qubits[index]
    }
}

impl<'a> ops::IndexMut<usize> for QuantumRegister<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.qubits[index]
    }
}

impl<'a> fmt::Display for QuantumGate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
