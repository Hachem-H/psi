use libpsi_core::*;
use std::fmt;

pub mod cli;
pub use cli::*;

pub trait Visualizer<'a>
where
    Self: fmt::Display,
{
    fn new(circuit: &'a QuantumCircuit) -> Self;
    fn render(&mut self);
}
