use libpsi_core::*;

fn main() {
    let quantum_registers = [
        QuantumRegister::new("qr0", &["q0", "q1", "q2", "q3"]),
        QuantumRegister::new("qr1", &["k0", "k1", "k2", "k3"]),
    ];

    let classical_registers = [
        ClassicalRegister::new("cr0", &["c0", "c1", "c2", "c3"]),
        ClassicalRegister::new("cr1", &["a0", "a1", "a2", "a3"]),
    ];

    QuantumCircuit::new(&quantum_registers, &classical_registers);
}
