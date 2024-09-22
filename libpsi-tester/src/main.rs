use libpsi::*;

fn main() {
    let bit_1 = qubit![(1.0, 2.0), (1.0, 2.0)];
    let bit_2 = qubit![(1.0, 2.0), (1.0, 2.0)];
    let register: QuantumRegister = quantum_register![bit_1, bit_2];
    println!("{}", register);
}
