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
        self.renderer.draw_quad(Quad::new(2, 2, 2, 2), false);
        self.renderer.draw_quad(Quad::new(5, 10, 6, 3), true);
        self.renderer.draw_quad(Quad::new(17, 4, 17, 10), true);
        self.renderer.draw_quad(Quad::new(15, 7, 7, 4), false);

        self.renderer.draw_vline(4, 4, 5, true);
        self.renderer.draw_hline(3, 6, 5, false);

        self.renderer.draw_text(4, 15, "This is some text.");
        println!("{}", self.renderer.to_string());
    }
}
