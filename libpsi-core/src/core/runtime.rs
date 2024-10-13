use super::{ClassicalRegister, QuantumCircuit, QuantumRegister, QuantumState};

pub struct RuntimeData {
    pub quantum_register: QuantumRegister,
    pub classical_states: ClassicalRegister,
}

impl RuntimeData {
    pub fn new(quantum_register: QuantumRegister, classical_states: ClassicalRegister) -> Self {
        RuntimeData {
            quantum_register,
            classical_states,
        }
    }
}

pub trait Runtime<'a> {
    fn new(circuit: &'a QuantumCircuit) -> Self;
    fn execute(&self, repeat: usize) -> Vec<RuntimeData>;
    fn get_state(&self) -> QuantumState;
    fn get_circuit(&self) -> &QuantumCircuit;
}
