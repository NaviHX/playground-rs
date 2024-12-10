use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign},
};

use num::{
    traits::{ConstOne, ConstZero},
    One, Zero,
};

pub struct Modular<T>(T, T);

impl<T> Modular<T> {
    pub fn new(n: T, m: T) -> Self {
        Self(n, m)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn modular(&self) -> &T {
        &self.1
    }
}

impl<T: Copy> Copy for Modular<T> {}

impl<T: Clone> Clone for Modular<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T: Add<Output = T> + Rem<Output = T> + Clone> Add for Modular<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % rhs.1.clone(), rhs.1)
    }
}

impl<T: Add<Output = T> + Rem<Output = T> + Clone> Add<T> for Modular<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self((self.0 + rhs) % self.1.clone(), self.1)
    }
}

impl<T: AddAssign + RemAssign + Clone> AddAssign for Modular<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 %= rhs.1.clone();
        self.1 = rhs.1;
    }
}

impl<T: AddAssign + RemAssign + Clone> AddAssign<T> for Modular<T> {
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs;
        self.0 %= self.1.clone();
    }
}

impl<T: Sum + Clone + Add<Output = T> + Rem<Output = T>> Sum for Modular<T>
where
    Modular<T>: Zero,
{
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let first = iter.next();

        if let Some(first) = first {
            iter.fold(first, |a, b| Self((a.0 + b.0) % a.1.clone(), a.1))
        } else {
            Self::zero()
        }
    }
}

impl<T: Mul<Output = T> + Rem<Output = T> + Clone> Mul for Modular<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % rhs.1.clone(), rhs.1)
    }
}

impl<T: Mul<Output = T> + Rem<Output = T> + Clone> Mul<T> for Modular<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self((self.0 * rhs) % self.1.clone(), self.1)
    }
}

impl<T: MulAssign + RemAssign + Clone> MulAssign for Modular<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 %= rhs.1.clone();
        self.1 = rhs.1;
    }
}

impl<T: MulAssign + RemAssign + Clone> MulAssign<T> for Modular<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
        self.0 %= self.1.clone();
    }
}

impl<T: One + Clone + Mul<Output = T> + Add<Output = T> + Rem<Output = T>> One for Modular<T> {
    fn one() -> Self {
        Self(T::one(), T::one())
    }
}

impl<T: Zero + Clone + Mul<Output = T> + Add<Output = T> + Rem<Output = T>> Zero for Modular<T> {
    fn zero() -> Self {
        Self(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T: ConstOne + Clone + Mul<Output = T> + Add<Output = T> + Rem<Output = T>> ConstOne
    for Modular<T>
{
    const ONE: Self = Self(T::ONE, T::ONE);
}

impl<T: ConstZero + Clone + Mul<Output = T> + Add<Output = T> + Rem<Output = T>> ConstZero
    for Modular<T>
{
    const ZERO: Self = Self(T::ZERO, T::ZERO);
}
