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

use crate::{renderer::*, Visualizer};
use libpsi_core::QuantumCircuit;

#[allow(unused)]
pub struct HorizontalCLIVisualizer<'a> {
    circuit: &'a QuantumCircuit,
    renderer: CLIRenderer,
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
        let quad = Quad::new(2, 2, 5, 5);
        let point = (10usize, 0usize);

        self.renderer.draw_quad(quad, false);
        self.renderer.connect(quad, point);
        println!("{}", self.renderer.to_string());
    }
}
