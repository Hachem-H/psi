use crate::{complex, matrix, QuantumGate};

#[rustfmt::skip]
lazy_static::lazy_static! {
    pub static ref HADAMARD: QuantumGate = matrix!([complex!(1.0, 0.0), complex!( 1.0, 0.0)];
                                                   [complex!(1.0, 0.0), complex!(-1.0, 0.0)]) *
                                           complex!(1.0/2.0_f64.sqrt(), 0.0);

    pub static ref PAULI_X: QuantumGate = matrix!([complex!(0.0, 0.0), complex!(1.0, 0.0)];
                                                  [complex!(1.0, 0.0), complex!(0.0, 0.0)]);

    pub static ref PAULI_Y: QuantumGate = matrix!([complex!(0.0, 0.0), complex!(0.0, -1.0)];
                                                  [complex!(0.0, 1.0), complex!(0.0,  0.0)]);

    pub static ref PAULI_Z: QuantumGate = matrix!([complex!(1.0, 0.0), complex!( 0.0, 0.0)];
                                                  [complex!(0.0, 0.0), complex!(-1.0, 0.0)]);

    pub static ref CNOT: QuantumGate = matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                                               [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                                               [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)];
                                               [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)]);
}
