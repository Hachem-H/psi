use crate::{complex, matrix, QuantumGate};

#[rustfmt::skip]
lazy_static::lazy_static! {
    pub static ref HADAMARD: QuantumGate<'static> = QuantumGate {
        name: "H",
        matrix: matrix!([complex!(1.0, 0.0), complex!( 1.0, 0.0)];
                        [complex!(1.0, 0.0), complex!(-1.0, 0.0)]) *
                complex!(1.0/2.0_f64.sqrt(), 0.0)
    };

    pub static ref PAULI_X: QuantumGate<'static> = QuantumGate {
        name: "Pauli-X",
        matrix: matrix!([complex!(0.0, 0.0), complex!(1.0, 0.0)];
                        [complex!(1.0, 0.0), complex!(0.0, 0.0)])
    };

    pub static ref PAULI_Y: QuantumGate<'static> = QuantumGate {
        name: "Pauli-Y", 
        matrix: matrix!([complex!(0.0, 0.0), complex!(0.0, -1.0)];
                        [complex!(0.0, 1.0), complex!(0.0,  0.0)])
    };

    pub static ref PAULI_Z: QuantumGate<'static> = QuantumGate {
        name: "Pauli-Z", 
        matrix: matrix!([complex!(1.0, 0.0), complex!( 0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(-1.0, 0.0)])
    };

    pub static ref CNOT: QuantumGate<'static> = QuantumGate {
        name: "CNOT", 
        matrix: matrix!([complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0)];
                        [complex!(0.0, 0.0), complex!(0.0, 0.0), complex!(1.0, 0.0), complex!(0.0, 0.0)])
    };
}
