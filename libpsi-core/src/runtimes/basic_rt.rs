use crate::{
    ColumnVector, Complex, ExecutationData, Numeric, QuantumBit, QuantumCircuit, QuantumRegister,
    Runtime, RuntimeBase, Vector,
};

pub struct BasicRT {
    circuit: QuantumCircuit,
}

impl RuntimeBase for BasicRT {
    fn get_circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }
}

impl BasicRT {
    fn calculate_state(state: &ColumnVector<Complex<f64>>, n: usize) -> QuantumBit {
        let num_states = 1 << n;
        let half_states = num_states >> 1;

        let mut alpha = Complex::zero();
        let mut beta = Complex::zero();

        for i in 0..num_states {
            if i < half_states {
                alpha += state[i];
            } else {
                beta += state[i];
            }
        }

        QuantumBit::new(vec![alpha, beta])
    }

    fn calculate_probabilities(state: &ColumnVector<Complex<f64>>, n: usize) -> (f64, f64) {
        let num_states = 1 << n;
        let half_states = num_states >> 1;

        let mut state_0 = 0.0;
        let mut state_1 = 0.0;

        for i in 0..num_states {
            let prob = state[i].norm() * state[i].norm();
            if i < half_states {
                state_0 += prob;
            } else {
                state_1 += prob;
            }
        }

        (state_0, state_1)
    }
}

impl Runtime for BasicRT {
    fn new(circuit: QuantumCircuit) -> BasicRT {
        BasicRT { circuit }
    }

    fn execute(&self, repeat: usize) -> Vec<ExecutationData> {
        let mut result: Vec<ExecutationData> = Vec::with_capacity(repeat);
        for _ in 0..repeat {
            let mut executation_data = ExecutationData::new(
                self.circuit.get_quantum_bits(),
                self.circuit.get_classical_registers(),
            );

            for instruction in self.circuit.get_instructions() {
                let mut control_bits: Vec<QuantumBit> = Vec::new();
                let mut target_bits: Vec<QuantumBit> = Vec::new();
                let mut quantum_bits: Vec<QuantumBit> = Vec::new();

                for index in instruction.control_indices.clone() {
                    control_bits.push(executation_data.quantum_states.clone()[index].clone());
                    quantum_bits.push(executation_data.quantum_states.clone()[index].clone());
                }

                for index in instruction.target_indices.clone() {
                    target_bits.push(executation_data.quantum_states.clone()[index].clone());
                    quantum_bits.push(executation_data.quantum_states.clone()[index].clone());
                }

                if control_bits.len() > 0 {
                    let control_register = QuantumRegister::from(&mut control_bits);
                    let control_register_state_vector = control_register.get_state();
                    let (control_register_state_0, control_register_state_1) =
                        BasicRT::calculate_probabilities(
                            &control_register_state_vector,
                            control_register.get_bits().len(),
                        );

                    if control_register_state_0 > control_register_state_1 {
                        continue;
                    }
                }

                let target_register = QuantumRegister::from(&mut quantum_bits);
                let target_state_vector = target_register.apply(&instruction.gate);
                let target_state =
                    BasicRT::calculate_state(&target_state_vector, quantum_bits.len());
                executation_data.quantum_states[instruction.target_indices.clone()[0]] =
                    target_state;
            }

            let final_quantum_register =
                QuantumRegister::from(&mut executation_data.quantum_states.clone());
            final_quantum_register.measure(&mut executation_data.classical_states);
            result.push(executation_data);
        }
        result
    }
}
