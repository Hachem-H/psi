use libpsi_core::*;
use libpsi_visualizer::*;

fn main() {
    let mut circuit = QuantumCircuit::new(2, 2);
    circuit.apply(&gates::HADAMARD, &[], &[0]);
    circuit.apply(&gates::CNOT, &[0], &[1]);

    let mut visualizer = HorizontalCLIVisualizer::new(&circuit);
    visualizer.render();

    let runtime = BasicRT::new(&circuit);
    runtime.execute(1024);
}
