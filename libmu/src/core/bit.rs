use crate::{ColumnVector, Complex, QuantumGate};

pub type QuantumBit = ColumnVector<Complex<f64>>;
pub type ClassicalBit = bool;

impl QuantumBit {
    pub fn apply_gate(&mut self, _gate: &QuantumGate) {
        todo!();
    }
}
