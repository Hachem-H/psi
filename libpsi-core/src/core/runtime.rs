use super::{ClassicalRegister, QuantumBit, QuantumCircuit};

#[allow(unused)]
pub struct ExecutationData {
    pub quantum_states: Vec<QuantumBit>,
    pub classical_states: ClassicalRegister,
}

impl ExecutationData {
    pub fn new(quantum_states: Vec<QuantumBit>, classical_states: ClassicalRegister) -> Self {
        ExecutationData {
            quantum_states,
            classical_states,
        }
    }
}

pub trait RuntimeBase {
    fn get_circuit(&self) -> &QuantumCircuit;
}

pub trait Runtime<T: RuntimeBase = Self> {
    fn new(circuit: QuantumCircuit) -> Self;
    fn execute(&self, repeat: usize) -> Vec<ExecutationData>;
}
