use libmu::*;

fn main() {
    #[rustfmt::skip]
    let matrix = matrix![[1.0, 2.0, 3.0]; 
                         [4.0, 5.0, 6.0];
                         [7.0, 8.0, 9.0]];
    let vector = row_vector![1.0, 2.0, 3.0];

    let result = vector.mul_matrix(&matrix).unwrap();

    println!("Matrix: \n{}", matrix);
    println!("Vector: \n{}", vector);
    println!("Result: \n{}", result);
}
