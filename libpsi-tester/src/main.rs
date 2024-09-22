use libpsi::*;

fn main() {
    let mut quantum_register = quantum_register![QuantumBit::state_0()];
    let mut classical_register = ClassicalRegister::new(1);

    quantum_register.measure(&mut classical_register);
    println!("|{}>", classical_register[0]);

    quantum_register.apply(&gates::HADAMARD, 0);
    quantum_register.measure(&mut classical_register);
    println!("|{}>", classical_register[0]);

    quantum_register.apply(&gates::HADAMARD, 0);
    quantum_register.measure(&mut classical_register);
    println!("|{}>", classical_register[0]);

    quantum_register.apply(&gates::HADAMARD, 0);
    quantum_register.measure(&mut classical_register);
    println!("|{}>", classical_register[0]);
}
