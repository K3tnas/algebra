use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Monoid: Clone + PartialEq + Add<Output = Self> + AddAssign {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait Group: Monoid + Sub<Output = Self> + SubAssign + Neg<Output = Self> {}

pub trait Ring: Group + Mul<Output = Self> + MulAssign {
    fn one() -> Self;
    fn is_one(&self) -> bool;
    fn pow(self, mut exp: u32) -> Self {
        let mut result = Self::one();
        let mut base = self;
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base.clone();
            }

            exp /= 2;

            if exp > 0 {
                base *= base.clone();
            }
        }
        result
    }
}

pub trait EuclideanDomain: Ring {
    fn norm(&self) -> u32;
    fn div_rem(self, rhs: Self) -> (Self, Self);

    fn rem(self, other: Self) -> Self {
        self.div_rem(other).1
    }

    fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();
        while b != Self::zero() {
            let r = a.rem(b.clone());
            a = b;
            b = r;
        }
        a
    }

    fn lcm(&self, other: &Self) -> Self {
        let gcd = self.gcd(other);
        (self.clone() * other.clone()).div_rem(gcd).0
    }
}

pub trait Field: EuclideanDomain + Div<Output = Self> + DivAssign {
    fn inv(self) -> Self;
    fn powi(self, n: i32) -> Self {
        if n >= 0 {
            self.pow(n as u32)
        } else {
            self.inv().pow((-n) as u32)
        }
    }
}
