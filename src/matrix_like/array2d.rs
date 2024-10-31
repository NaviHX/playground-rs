use num::{
    traits::{ConstOne, ConstZero},
    One, Zero,
};

use super::{
    AddIdentityMatrixLike, FromArray2DLike, MatrixLike, MulIdentityMatrixLike, NewMatrixLike,
    SizeTransformMatrixLike,
};

pub struct Array2D<const ROW: usize, const COL: usize, T>(pub [[T; COL]; ROW]);

impl<const ROW: usize, const COL: usize, T: Copy> Copy for Array2D<ROW, COL, T> {}
impl<const ROW: usize, const COL: usize, T: Clone> Clone for Array2D<ROW, COL, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<const ROW: usize, const COL: usize, T> MatrixLike<ROW, COL, T> for Array2D<ROW, COL, T> {
    fn get_opt(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(x).and_then(|row| row.get(y))
    }

    fn get_mut_opt(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.0.get_mut(x).and_then(|row| row.get_mut(y))
    }
}

impl<const ROW: usize, const COL: usize, T: ConstZero + Copy> NewMatrixLike
    for Array2D<ROW, COL, T>
{
    fn new() -> Self {
        Self([[T::ZERO; COL]; ROW])
    }
}

impl<const ROW: usize, T: One + ConstOne + Zero + ConstZero + Copy> MulIdentityMatrixLike
    for Array2D<ROW, ROW, T>
{
    fn one() -> Self {
        let mut buf = [[T::ZERO; ROW]; ROW];

        #[allow(clippy::needless_range_loop)]
        for i in 0..ROW {
            buf[i][i] = T::ONE;
        }

        Self(buf)
    }
}

impl<const ROW: usize, const COL: usize, T: Zero + ConstZero + Copy> AddIdentityMatrixLike
    for Array2D<ROW, COL, T>
{
    fn zero() -> Self {
        Self([[T::ZERO; COL]; ROW])
    }
}

impl<
        const ROW: usize,
        const COL: usize,
        const NEW_ROW: usize,
        const NEW_COL: usize,
        T: Copy + Zero + ConstZero,
    > SizeTransformMatrixLike<NEW_ROW, NEW_COL, T> for Array2D<ROW, COL, T>
{
    type Transformed = Array2D<NEW_ROW, NEW_COL, T>;
}

impl<const ROW: usize, const COL: usize, T: Copy> FromArray2DLike<ROW, COL, T>
    for Array2D<ROW, COL, T>
{
    fn from_array2d(array: Array2D<ROW, COL, T>) -> Self {
        array
    }
}
