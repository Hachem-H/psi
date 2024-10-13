/*
 NOTE(Hachem): This is what I would like the output to be like

            ┌───┐    ┌───────┐    ┌─────────┐
 |q0>: ─────┤ H ├────┤ C-NOT ├────┤ MEASURE ├───────────────────▓
            └───┘    └───┬───┘    └────┬────┘
                         │             │         ┌─────────┐
 |q1>: ──────────────────■─────────────┼─────────┤ MEASURE ├────▓
                                       │         └────┬────┘
                                       │              │
   c0: ════════════════════════════════■══════════════╪═════════▓
   c1: ═══════════════════════════════════════════════■═════════▓
*/

use core::fmt;

use crate::{renderer::*, Visualizer};
use libpsi_core::QuantumCircuit;

#[allow(unused)]
pub struct HorizontalCLIVisualizer<'a> {
    circuit: &'a QuantumCircuit<'a>,
    renderer: CLIRenderer,
}

impl<'a> fmt::Display for HorizontalCLIVisualizer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.renderer.to_string())
    }
}

impl<'a> Visualizer<'a> for HorizontalCLIVisualizer<'a> {
    fn new(circuit: &'a QuantumCircuit) -> HorizontalCLIVisualizer<'a> {
        let terminal_width = term_size::dimensions().unwrap().0;
        HorizontalCLIVisualizer {
            circuit,
            renderer: CLIRenderer::new(terminal_width),
        }
    }

    fn render(&mut self) {
        println!("{}", self);
    }
}
