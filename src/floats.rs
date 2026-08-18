use crate::algstructs::{EuclideanDomain, Field, Group, Monoid, Ring};

impl Monoid for f64 {
    fn zero() -> Self {
        0.0
    }

    fn is_zero(&self) -> bool {
        *self == 0.0
    }
}

impl Group for f64 {}

impl Ring for f64 {
    fn one() -> Self {
        1.0
    }

    fn is_one(&self) -> bool {
        *self == 1.0
    }
}

impl EuclideanDomain for f64 {
    fn norm(&self) -> u32 {
        0
    }

    fn div_rem(self, rhs: Self) -> (Self, Self) {
        (self / rhs, 0.0)
    }
}

impl Field for f64 {
    fn inv(self) -> Self {
        1.0 / self
    }
}
