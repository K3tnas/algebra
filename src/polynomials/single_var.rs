use crate::{
    algstructs::{EuclideanDomain, Field, Monoid, Ring},
    polynomials::Polynomial,
};

impl<T: Ring> Polynomial<T> {
    pub fn deg(&self) -> u32 {
        self.lt().powers[0]
    }
}

impl<T: Field> EuclideanDomain for Polynomial<T> {
    fn norm(&self) -> u32 {
        self.deg()
    }

    fn div_rem(self, divisor: Self) -> (Self, Self) {
        assert!(!divisor.is_zero());
        let mut remainder = self;
        let mut quotient = Self::zero();

        while !remainder.is_zero() && remainder.deg() >= divisor.deg() {
            let a = remainder.lt() / quotient.lt();
            quotient += a.clone();
            remainder -= divisor.clone() * a;
        }

        (quotient, remainder)
    }
}
