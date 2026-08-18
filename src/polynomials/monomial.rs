use crate::algstructs::{EuclideanDomain, Ring};

#[derive(Clone, PartialEq, Debug)]
#[derive(Copy)]
pub struct Monomial<T: Ring, const N: usize> {
    pub coef: T,
    pub powers: [u32; N],
}

impl<T: EuclideanDomain, const N: usize> Monomial<T, N> {
    pub fn divides(&self, rhs: &Self) -> bool {
        if self.coef.is_zero() {
            return false;
        }

        for i in 0..N {
            if rhs.powers[i] < self.powers[i] {
                return false;
            }
        }

        true
    }
}
