use crate::{Instruction, QuantumCircuit, QuantumState, Runtime, RuntimeData};

pub struct BasicRT<'a> {
    circuit: &'a QuantumCircuit<'a>,
}

impl<'a> Runtime<'a> for BasicRT<'a> {
    fn new(circuit: &'a QuantumCircuit) -> BasicRT<'a> {
        BasicRT { circuit }
    }

    fn execute(&self, repeat: usize) -> Vec<RuntimeData> {
        let mut result: Vec<RuntimeData> = Vec::with_capacity(repeat);
        for _ in 0..repeat {
            let executation_data = RuntimeData::new(
                self.circuit.get_quantum_register(),
                self.circuit.get_classical_registers(),
            );

            for instruction in self.circuit.get_instructions() {
                match instruction {
                    Instruction::ApplyGate(_gate, _control_bits, _target) => print!(""),
                    Instruction::Measure(_bit) => print!(""),
                }
            }

            result.push(executation_data);
        }
        result
    }

    fn get_state(&self) -> QuantumState {
        todo!()
    }

    fn get_circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }
}
