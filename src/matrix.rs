#![allow(unused)]

use std::{
    iter::Sum,
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use num::{
    traits::{ConstOne, ConstZero},
    One, Zero,
};

use crate::matrix_like::{
    AddIdentityMatrixLike, Array2D, FromArray2DLike, MatrixLike, MulIdentityMatrixLike,
    SizeTransformMatrixLike,
};

pub struct Matrix<const ROW: usize, const COL: usize, E, C: MatrixLike<ROW, COL, E>>(
    pub C,
    PhantomData<E>,
);

impl<const ROW: usize, const COL: usize, T: Copy, C: Copy + MatrixLike<ROW, COL, T>> Copy
    for Matrix<ROW, COL, T, C>
{
}
impl<const ROW: usize, const COL: usize, T: Clone, C: Clone + MatrixLike<ROW, COL, T>> Clone
    for Matrix<ROW, COL, T, C>
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<const ROW: usize, const COL: usize, E, C: MatrixLike<ROW, COL, E>> MatrixLike<ROW, COL, E>
    for Matrix<ROW, COL, E, C>
{
    fn get_opt(&self, x: usize, y: usize) -> Option<&E> {
        self.0.get_opt(x, y)
    }

    fn get_mut_opt(&mut self, x: usize, y: usize) -> Option<&mut E> {
        self.0.get_mut_opt(x, y)
    }
}

impl<
        const ROW: usize,
        E: Copy + Sum + Add<Output = E> + Mul<Output = E>,
        C: MulIdentityMatrixLike
            + MatrixLike<ROW, ROW, E>
            + SizeTransformMatrixLike<ROW, ROW, E, Transformed = C>
            + FromArray2DLike<ROW, ROW, E>,
    > One for Matrix<ROW, ROW, E, C>
{
    fn one() -> Self {
        Self(MulIdentityMatrixLike::one(), PhantomData)
    }
}

impl<
        const ROW: usize,
        const COL: usize,
        E: Zero + ConstZero + Copy + PartialEq,
        C: AddIdentityMatrixLike + MatrixLike<ROW, COL, E>,
    > Zero for Matrix<ROW, COL, E, C>
{
    fn zero() -> Self {
        Self(AddIdentityMatrixLike::zero(), PhantomData)
    }

    fn is_zero(&self) -> bool {
        (0..ROW).all(|x| (0..COL).all(|y| *self.get(x, y) == E::ZERO))
    }
}

impl<const ROW: usize, const COL: usize, E, C: MatrixLike<ROW, COL, E>> Matrix<ROW, COL, E, C> {
    pub fn from_container(container: C) -> Self {
        Self(container, PhantomData)
    }
}

impl<const ROW: usize, const COL: usize, E> Matrix<ROW, COL, E, Array2D<ROW, COL, E>> {
    pub fn from_array(array: [[E; COL]; ROW]) -> Self {
        Matrix(Array2D(array), PhantomData)
    }
}

impl<const ROW: usize, const COL: usize, E: Copy> Matrix<ROW, COL, E, Array2D<ROW, COL, E>> {
    pub fn copy_from_array(array: &[[E; COL]; ROW]) -> Matrix<ROW, COL, E, Array2D<ROW, COL, E>> {
        let mut buf: [[MaybeUninit<E>; COL]; ROW] = [[const { MaybeUninit::uninit() }; COL]; ROW];
        unsafe {
            let buf: &mut [[E; COL]; ROW] = &mut *((&mut buf as *mut _) as *mut [[E; COL]; ROW]);
            buf.copy_from_slice(array);
        }

        let buf = unsafe { *((&buf as *const _) as *const [[E; COL]; ROW]) };

        Matrix(Array2D(buf), PhantomData)
    }
}

impl<const ROW: usize, const COL: usize, E: Clone + Copy>
    Matrix<ROW, COL, E, Array2D<ROW, COL, E>>
{
    pub fn clone_from_array(array: &[[E; COL]; ROW]) -> Matrix<ROW, COL, E, Array2D<ROW, COL, E>> {
        let mut buf: [[MaybeUninit<E>; COL]; ROW] = [[const { MaybeUninit::uninit() }; COL]; ROW];
        unsafe {
            let buf: &mut [[E; COL]; ROW] = &mut *((&mut buf as *mut _) as *mut [[E; COL]; ROW]);
            buf.clone_from_slice(array);
        }

        let buf = unsafe { *((&buf as *const _) as *const [[E; COL]; ROW]) };

        Matrix(Array2D(buf), PhantomData)
    }
}

impl<
        const ROW: usize,
        const COL: usize,
        const COL2: usize,
        E: Mul<E, Output = E> + Add<E, Output = E> + Sum<E> + Copy,
        CL: MatrixLike<ROW, COL, E> + SizeTransformMatrixLike<ROW, COL2, E>,
        CR: MatrixLike<COL, COL2, E>,
    > Mul<Matrix<COL, COL2, E, CR>> for Matrix<ROW, COL, E, CL>
where
    CL::Transformed: FromArray2DLike<ROW, COL2, E>,
{
    type Output = Matrix<ROW, COL2, E, CL::Transformed>;

    fn mul(self, rhs: Matrix<COL, COL2, E, CR>) -> Self::Output {
        let mut array: [[MaybeUninit<E>; COL2]; ROW] =
            [[const { MaybeUninit::uninit() }; COL2]; ROW];

        for id in 0..ROW * COL2 {
            let (x, y) = (id / COL2, id % COL2);
            array[x][y].write(
                (0..COL)
                    .map(|z| ((x, z), (z, y)))
                    .map(|((xl, yl), (xr, yr))| (self.get(xl, yl), rhs.get(xr, yr)))
                    .map(|(a, b)| *a * *b)
                    .sum(),
            );
        }

        let array: [[E; COL2]; ROW] = unsafe { *((&array as *const _) as *const _) };
        let mut new_container = CL::Transformed::from_array2d(Array2D(array));

        Matrix::from_container(new_container)
    }
}

impl<
        const ROW: usize,
        const COL: usize,
        E: Add<E, Output = E> + Copy,
        CL: MatrixLike<ROW, COL, E>,
        CR: MatrixLike<ROW, COL, E>,
    > Add<Matrix<ROW, COL, E, CR>> for Matrix<ROW, COL, E, CL>
{
    type Output = Matrix<ROW, COL, E, CL>;

    fn add(mut self, rhs: Matrix<ROW, COL, E, CR>) -> Self::Output {
        #[allow(clippy::needless_range_loop)]
        for x in 0..ROW {
            for y in 0..COL {
                *self.get_mut(x, y) = *self.get(x, y) + *rhs.get(x, y);
            }
        }

        self
    }
}

impl<
        const ROW: usize,
        const COL: usize,
        E: Mul<E, Output = E> + Add<E, Output = E> + Sum<E> + Copy,
        CL: MatrixLike<ROW, COL, E>,
        CR: MatrixLike<COL, COL, E>,
    > MulAssign<Matrix<COL, COL, E, CR>> for Matrix<ROW, COL, E, CL>
{
    fn mul_assign(&mut self, rhs: Matrix<COL, COL, E, CR>) {
        let mut array: [[MaybeUninit<E>; COL]; ROW] = [[const { MaybeUninit::uninit() }; COL]; ROW];

        for id in 0..ROW * COL {
            let (x, y) = (id / COL, id % COL);
            array[x][y].write(
                (0..COL)
                    .map(|z| ((x, z), (z, y)))
                    .map(|((xl, yl), (xr, yr))| (self.get(xl, yl), rhs.get(xr, yr)))
                    .map(|(a, b)| *a * *b)
                    .sum(),
            );
        }

        let array: [[E; COL]; ROW] = unsafe { *((&array as *const _) as *const _) };
        for id in 0..ROW * COL {
            let (x, y) = (id / COL, id % ROW);
            *self.get_mut(x, y) = array[x][y];
        }
    }
}

impl<
        const ROW: usize,
        const COL: usize,
        E: Add<E, Output = E> + Copy,
        CL: MatrixLike<ROW, COL, E>,
        CR: MatrixLike<ROW, COL, E>,
    > AddAssign<Matrix<ROW, COL, E, CR>> for Matrix<ROW, COL, E, CL>
{
    fn add_assign(&mut self, rhs: Matrix<ROW, COL, E, CR>) {
        #[allow(clippy::needless_range_loop)]
        for x in 0..ROW {
            for y in 0..COL {
                *self.get_mut(x, y) = *self.get(x, y) + *rhs.get(x, y);
            }
        }
    }
}
