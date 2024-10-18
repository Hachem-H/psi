use super::{ClassicalRegister, QuantumRegister};

#[allow(unused)]
pub struct QuantumCircuit<'a> {
    quantum_registers: &'a [QuantumRegister<'a>],
    classical_registers: &'a [ClassicalRegister<'a>],
}

impl<'a> QuantumCircuit<'a> {
    pub fn new(
        quantum_registers: &'a [QuantumRegister<'a>],
        classical_registers: &'a [ClassicalRegister<'a>],
    ) -> QuantumCircuit<'a> {
        QuantumCircuit {
            quantum_registers,
            classical_registers,
        }
    }
}
