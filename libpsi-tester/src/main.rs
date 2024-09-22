use libpsi::*;

fn main() {
    let mut register = quantum_register![qubit![(1.0, 0.0), (0.0, 0.0)]];
    let hadamard = matrix!([Complex::one(), Complex::one()]; [Complex::one(), -Complex::one()])
        .scale(complex!(1.0 / f64::sqrt(2.0), 0.0));
    println!("Before:\n{}", register[0]);
    register.apply(&hadamard, 0);
    println!("\nAfter H Gate:\n{}", register[0]);
}
