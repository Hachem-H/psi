use super::{ClassicalRegister, QuantumBit, QuantumGate};

#[allow(dead_code)]
struct QuantumCircuitInstruction {
    target_indices: Vec<i32>,
    control_indices: Vec<i32>,
    gate: QuantumGate,
}

#[allow(dead_code)]
pub struct QuantumCircuit {
    quantum_bits: Vec<QuantumBit>,
    classical_register: ClassicalRegister,
    instructions: Vec<QuantumCircuitInstruction>,
}

impl QuantumCircuit {
    pub fn new(quantum_bit_count: usize, classical_bit_count: usize) -> QuantumCircuit {
        QuantumCircuit {
            quantum_bits: Vec::with_capacity(quantum_bit_count),
            classical_register: ClassicalRegister::new(classical_bit_count),
            instructions: Vec::new(),
        }
    }
}
