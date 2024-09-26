use super::{ClassicalRegister, QuantumBit, QuantumGate};

#[allow(dead_code)]
struct QuantumCircuitInstruction {
    target_indices: Vec<usize>,
    control_indices: Vec<usize>,
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

    pub fn apply(&mut self, gate: &QuantumGate, control_bits: &[usize], target_bits: &[usize]) {
        self.instructions.push(QuantumCircuitInstruction {
            gate: gate.clone(),
            target_indices: target_bits.to_vec(),
            control_indices: control_bits.to_vec(),
        });
    }
}
