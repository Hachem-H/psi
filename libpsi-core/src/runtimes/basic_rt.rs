use crate::{ExecutionData, QuantumCircuit, Runtime, RuntimeBase};

pub struct BasicRT<'a> {
    circuit: &'a QuantumCircuit,
}

impl<'a> RuntimeBase for BasicRT<'a> {
    fn get_circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }
}

impl<'a> Runtime<'a> for BasicRT<'a> {
    fn new(circuit: &'a QuantumCircuit) -> BasicRT {
        BasicRT { circuit }
    }

    fn execute(&self, repeat: usize) -> Vec<ExecutionData> {
        let mut result: Vec<ExecutionData> = Vec::with_capacity(repeat);
        for _ in 0..repeat {
            let executation_data = ExecutionData::new(
                self.circuit.get_quantum_register(),
                self.circuit.get_classical_registers(),
            );

            for instruction in self.circuit.get_instructions() {
                println!("{} on q{}", instruction.gate.0, instruction.target_bit);
            }

            result.push(executation_data);
        }
        result
    }
}
