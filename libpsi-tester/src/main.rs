use libpsi_core::*;

fn count_qubit_states(measurements: &[Vec<i32>], num_qubits: usize) -> Vec<i32> {
    let mut counts = vec![0; num_qubits * 2];
    for measurement in measurements {
        for (i, &state) in measurement.iter().enumerate() {
            if state == 0 {
                counts[i] += 1;
            } else {
                counts[num_qubits + i] += 1;
            }
        }
    }
    counts
}

fn main() {
    let mut circuit = QuantumCircuit::new(1, 1);
    circuit.apply(&gates::HADAMARD, &[], &[0]);

    let runtime = BasicRT::new(circuit);
    let execution_count = 1024;
    let executions = runtime.execute(execution_count);

    let mut measurements = Vec::new();
    for execution in executions {
        let bits = execution.classical_states.get_bits();
        measurements.push(bits.clone());
    }

    let counts = count_qubit_states(&measurements, 1);
    println!(
        "{:.2} * |0> + {:.2} * |1>",
        (counts[0] as f32) / execution_count as f32,
        (counts[1] as f32) / execution_count as f32
    );
}
