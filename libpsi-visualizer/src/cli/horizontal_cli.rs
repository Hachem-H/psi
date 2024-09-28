use crate::Visualizer;
use libpsi_core::QuantumCircuit;

pub struct HorizontalCLIVisualizer<'a> {
    circuit: &'a QuantumCircuit,
}

impl<'a> Visualizer<'a> for HorizontalCLIVisualizer<'a> {
    fn new(circuit: &'a QuantumCircuit) -> HorizontalCLIVisualizer<'a> {
        HorizontalCLIVisualizer { circuit }
    }

    // NOTE(Hachem): This is a temporary function
    fn render(&self) {
        for instruction in self.circuit.get_instructions() {
            print!("Applied {} on ", &instruction.gate.0);
            for bit_index in instruction.control_indices {
                print!("q{} ", bit_index);
            }
            for bit_index in instruction.target_indices {
                print!("q{} ", bit_index);
            }
            println!();
        }
    }
}
