use super::{ClassicalRegister, QuantumCircuit, QuantumRegister};

#[allow(unused)]
pub struct ExecutionData {
    pub quantum_register: QuantumRegister,
    pub classical_states: ClassicalRegister,
}

impl ExecutionData {
    pub fn new(quantum_register: QuantumRegister, classical_states: ClassicalRegister) -> Self {
        ExecutionData {
            quantum_register,
            classical_states,
        }
    }
}

pub trait RuntimeBase {
    fn get_circuit(&self) -> &QuantumCircuit;
}

pub trait Runtime<'a, T: RuntimeBase = Self> {
    fn new(circuit: &'a QuantumCircuit) -> Self;
    fn execute(&self, repeat: usize) -> Vec<ExecutionData>;
}
