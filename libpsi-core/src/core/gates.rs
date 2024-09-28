use crate::{complex, matrix, QuantumGate};

#[rustfmt::skip]
lazy_static::lazy_static! {
    pub static ref HADAMARD: QuantumGate = ("H", matrix!([complex!(1.0, 0.0), complex!( 1.0, 0.0)];
                                                         [complex!(1.0, 0.0), complex!(-1.0, 0.0)]) *
                                                 complex!(1.0/2.0_f64.sqrt(), 0.0));

    pub static ref PAULI_X: QuantumGate = ("Pauli-X", matrix!([complex!(0.0, 0.0), complex!(1.0, 0.0)];
                                                              [complex!(1.0, 0.0), complex!(0.0, 0.0)]));

    pub static ref PAULI_Y: QuantumGate = ("Pauli-Y", matrix!([complex!(0.0, 0.0), complex!(0.0, -1.0)];
                                                              [complex!(0.0, 1.0), complex!(0.0,  0.0)]));

    pub static ref PAULI_Z: QuantumGate = ("Pauli-Z", matrix!([complex!(1.0, 0.0), complex!( 0.0, 0.0)];
                                                              [complex!(0.0, 0.0), complex!(-1.0, 0.0)]));

    pub static ref CNOT: QuantumGate = ("CNOT", matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                                                        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                                                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)];
                                                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)]));
}
