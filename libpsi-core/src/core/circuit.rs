use super::{ClassicalRegister, QuantumGate, QuantumRegister};

#[derive(Clone)]
pub struct QuantumCircuitInstruction {
    pub target_bit: usize,
    pub control_indices: Vec<usize>,
    pub gate: QuantumGate,
}

pub struct QuantumCircuit {
    quantum_register: QuantumRegister,
    classical_register: ClassicalRegister,
    instructions: Vec<QuantumCircuitInstruction>,
}

impl QuantumCircuit {
    pub fn new(quantum_bit_count: usize, classical_bit_count: usize) -> QuantumCircuit {
        QuantumCircuit {
            quantum_register: QuantumRegister::new(quantum_bit_count),
            classical_register: ClassicalRegister::new(classical_bit_count),
            instructions: Vec::new(),
        }
    }

    pub fn apply(&mut self, gate: &QuantumGate, control_bits: &[usize], target_bit: usize) {
        self.instructions.push(QuantumCircuitInstruction {
            gate: gate.clone(),
            target_bit,
            control_indices: control_bits.to_vec(),
        });
    }

    pub fn get_quantum_register(&self) -> QuantumRegister {
        self.quantum_register.clone()
    }

    pub fn get_classical_registers(&self) -> ClassicalRegister {
        self.classical_register.clone()
    }

    pub fn get_instructions(&self) -> Vec<QuantumCircuitInstruction> {
        self.instructions.clone()
    }
}
