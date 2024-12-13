use playground_rs::{algebra::QuickPow, matrix::Matrix, matrix_like::MatrixLike};

fn main() {
    let n = 3;
    let v = Matrix::from_array([[1, 0]]);
    let transform = Matrix::from_array([[1, 1], [1, 0]]);
    let transform = transform.pow(n);
    let res = v * transform;

    println!("Fibonacci {n} is {}", res.get(0, 0))
}
