use libpsi_core::*;
use libpsi_visualizer::*;

fn main() {
    let mut circuit = QuantumCircuit::new(2, 2);
    circuit.execute(Instruction::ApplyGate(&gates::HADAMARD, &[], 0));
    circuit.execute(Instruction::ApplyGate(&gates::CNOT, &[0], 1));
    circuit.execute(Instruction::ApplyGate(&gates::HADAMARD, &[], 0));

    circuit.execute(Instruction::Measure(0));
    circuit.execute(Instruction::Measure(1));

    HorizontalCLIVisualizer::new(&circuit).render();
    BasicRT::new(&circuit).execute(1024);
}
