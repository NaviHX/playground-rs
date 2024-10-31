use std::ops::MulAssign;

use num::One;

pub trait QuickPow {
    fn pow(self, p: usize) -> Self;
}

impl<T: One + Clone + MulAssign> QuickPow for T {
    fn pow(self, mut p: usize) -> Self {
        assert!(p >= 1);
        let mut base = self;
        let mut product = Self::one();

        while p != 0 {
            if p & 1 != 0 {
                product *= base.clone();
            }

            base *= base.clone();
            p >>= 1;
        }

        product
    }
}
