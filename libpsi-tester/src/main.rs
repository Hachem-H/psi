use libpsi_core::*;

fn main() {
    let mut circuit = QuantumCircuit::new(2, 2);
    circuit.apply(&gates::CNOT, &[0], &[1]);

    let runtime = BasicRT::new(circuit);
    runtime.execute(1024);
}
