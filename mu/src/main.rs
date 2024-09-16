use libmu::Matrix;

fn main() {
    let mat1 = Matrix::<f32>::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    let mat2 = Matrix::<f32>::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

    let dot_product = mat1.dot(&mat2);
    let kronecker_product = mat1.kronecker(&mat2);

    println!("Matrix 1:\n{}", mat1);
    println!("Matrix 2:\n{}", mat2);

    if let Some(dp) = dot_product {
        println!("Dot Product Matrix:\n{}", dp);
    }

    println!("Kronecker Product Matrix:\n{}", kronecker_product);
}
