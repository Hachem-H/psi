use libpsi_core::*;

pub mod cli;
pub use cli::*;

pub trait Visualizer<'a> {
    fn new(circuit: &'a QuantumCircuit) -> Self;
    fn render(&mut self);
}
