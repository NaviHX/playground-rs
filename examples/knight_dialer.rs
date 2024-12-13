use playground_rs::{
    algebra::{Modular, QuickPow},
    matrix::Matrix,
    matrix_like::MatrixLike,
};

fn main() {
    // https://leetcode.com/problems/knight-dialer

    const MOD: usize = 1_000_000_007;
    let m = |v| Modular::new(v, MOD);
    let n = 3131;
    let mut res = Matrix::from_array([
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
        [m(1)],
    ]);
    let transform = Matrix::from_array([
        [m(0), m(0), m(0), m(0), m(0), m(1), m(0), m(1), m(0), m(0)],
        [m(0), m(0), m(0), m(0), m(0), m(0), m(1), m(0), m(1), m(0)],
        [m(0), m(0), m(0), m(1), m(0), m(0), m(0), m(1), m(0), m(0)],
        [m(0), m(0), m(1), m(0), m(0), m(0), m(0), m(0), m(1), m(1)],
        [m(0), m(0), m(0), m(0), m(0), m(0), m(0), m(0), m(0), m(0)],
        [m(1), m(0), m(0), m(0), m(0), m(0), m(1), m(0), m(0), m(1)],
        [m(0), m(1), m(0), m(0), m(0), m(1), m(0), m(0), m(0), m(0)],
        [m(1), m(0), m(1), m(0), m(0), m(0), m(0), m(0), m(0), m(0)],
        [m(0), m(1), m(0), m(1), m(0), m(0), m(0), m(0), m(0), m(0)],
        [m(0), m(0), m(0), m(1), m(0), m(1), m(0), m(0), m(0), m(0)],
    ]);

    let transform = transform.pow(n - 1);
    res = transform * res;

    let &sum = (0..10)
        .map(|i| *res.get(i, 0))
        .sum::<Modular<usize>>()
        .get();
    println!("There are {sum} possible numbers.")
}
