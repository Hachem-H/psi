use super::{ClassicalRegister, QuantumGate, QuantumRegister};

#[derive(Clone)]
pub enum Instruction<'a> {
    ApplyGate(&'a QuantumGate, &'a [usize], usize),
    Measure(usize),
}

pub struct QuantumCircuit<'a> {
    quantum_register: QuantumRegister,
    classical_register: ClassicalRegister,
    instructions: Vec<Instruction<'a>>,
}

impl<'a> QuantumCircuit<'a> {
    pub fn new(quantum_bit_count: usize, classical_bit_count: usize) -> QuantumCircuit<'a> {
        QuantumCircuit {
            quantum_register: QuantumRegister::new(quantum_bit_count),
            classical_register: ClassicalRegister::new(classical_bit_count),
            instructions: Vec::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction<'a>) {
        self.instructions.push(instruction);
    }

    pub fn get_quantum_register(&self) -> QuantumRegister {
        self.quantum_register.clone()
    }

    pub fn get_classical_registers(&self) -> ClassicalRegister {
        self.classical_register.clone()
    }

    pub fn get_instructions(&self) -> Vec<Instruction> {
        self.instructions.clone()
    }
}
