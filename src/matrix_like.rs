#![allow(unused)]

mod array2d;

pub use array2d::Array2D;

pub trait MatrixLike<const ROW: usize, const COL: usize, T> {
    fn get(&self, x: usize, y: usize) -> &T {
        self.get_opt(x, y).unwrap()
    }
    fn get_opt(&self, x: usize, y: usize) -> Option<&T>;

    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.get_mut_opt(x, y).unwrap()
    }
    fn get_mut_opt(&mut self, x: usize, y: usize) -> Option<&mut T>;
}

pub trait MulIdentityMatrixLike {
    fn one() -> Self;
}

pub trait AddIdentityMatrixLike {
    fn zero() -> Self;
}

pub trait NewMatrixLike {
    fn new() -> Self;
}

pub trait SizeTransformMatrixLike<const ROW: usize, const COL: usize, T> {
    type Transformed: MatrixLike<ROW, COL, T>;
}

pub trait FromArray2DLike<const ROW: usize, const COL: usize, T: Copy> {
    fn from_array2d(array: Array2D<ROW, COL, T>) -> Self;
}
