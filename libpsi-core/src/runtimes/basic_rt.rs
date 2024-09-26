use crate::{ClassicalRegister, ExecutationData, QuantumCircuit, Runtime, RuntimeBase};

pub struct BasicRT {
    circuit: QuantumCircuit,
}

impl RuntimeBase for BasicRT {
    fn get_circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }
}

impl Runtime for BasicRT {
    fn new(circuit: QuantumCircuit) -> BasicRT {
        BasicRT { circuit }
    }

    fn execute(&self, repeat: usize) -> Vec<ExecutationData> {
        let mut result: Vec<ExecutationData> = Vec::with_capacity(repeat);
        for _ in 0..repeat {
            let executation_data = ExecutationData::new(Vec::new(), ClassicalRegister::new(1));
            result.push(executation_data);
        }
        result
    }
}
