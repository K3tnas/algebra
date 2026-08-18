use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{
    algstructs::{Field, Monoid, Ring},
    polynomials::{Polynomial, monomial::Monomial, monomial_order::MonomialOrder},
};

// poly + poly = poly
// poly + monomial = poly
// poly + T = poly (nie mozemy implementować dla dowolnego T)
// monomial + monomial = monomial
// monomial + poly = poly

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Add for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn add(self, rhs: Self) -> Self::Output {
        rhs.into_iter().fold(self, Polynomial::add)
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Add<Monomial<T, N>> for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn add(mut self, rhs: Monomial<T, N>) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Add<T> for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn add(self, rhs: T) -> Self::Output {
        self + Monomial {
            coef: rhs,
            powers: [0; N],
        }
    }
}

impl<T: Ring, const N: usize> Add for Monomial<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut powers = [0; N];
        (0..N).for_each(|i| powers[i] = self.powers[i] + rhs.powers[i]);

        Monomial {
            coef: self.coef + rhs.coef,
            powers,
        }
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Add<Polynomial<T, N, O>> for Monomial<T, N> {
    type Output = Polynomial<T, N, O>;

    fn add(self, mut rhs: Polynomial<T, N, O>) -> Self::Output {
        rhs.insert(self);
        rhs
    }
}

// monomial += monomial
// poly += poly
// poly += mono
// poly += T

impl<T: Ring, const N: usize> AddAssign for Monomial<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        self.coef += rhs.coef;
        (0..N).for_each(|i| self.powers[i] += rhs.powers[i]);
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> AddAssign for Polynomial<T, N, O> {
    fn add_assign(&mut self, rhs: Self) {
        for m in rhs {
            self.insert(m);
        }
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> AddAssign<Monomial<T, N>>
    for Polynomial<T, N, O>
{
    fn add_assign(&mut self, rhs: Monomial<T, N>) {
        self.insert(rhs);
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> AddAssign<T> for Polynomial<T, N, O> {
    fn add_assign(&mut self, rhs: T) {
        self.insert(Monomial {
            coef: rhs,
            powers: [0; N],
        });
    }
}

// -monomial
// -poly

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Neg for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn neg(mut self) -> Self::Output {
        self.iter_mut().for_each(|m| m.coef *= -T::one());
        self
    }
}

impl<T: Ring, const N: usize> Neg for Monomial<T, N> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.coef *= -T::one();
        self
    }
}

// poly - poly = poly
// poly - monomial = poly
// poly - T = poly (nie mozemy implementować dla dowolnego T)
// monomial - monomial = monomial
// monomial - poly = poly

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Sub for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Sub<Monomial<T, N>> for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn sub(self, rhs: Monomial<T, N>) -> Self::Output {
        self + -rhs
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Sub<T> for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn sub(self, rhs: T) -> Self::Output {
        self + -rhs
    }
}

impl<T: Ring, const N: usize> Sub for Monomial<T, N> {
    type Output = Monomial<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Sub<Polynomial<T, N, O>> for Monomial<T, N> {
    type Output = Polynomial<T, N, O>;

    fn sub(self, rhs: Polynomial<T, N, O>) -> Self::Output {
        self + -rhs
    }
}

// poly -= poly
// poly -= mono
// poly -= T
// monomial -= monomial
impl<T: Ring, const N: usize, O: MonomialOrder<N>> SubAssign for Polynomial<T, N, O> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> SubAssign<Monomial<T, N>>
    for Polynomial<T, N, O>
{
    fn sub_assign(&mut self, rhs: Monomial<T, N>) {
        *self += -rhs;
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> SubAssign<T> for Polynomial<T, N, O> {
    fn sub_assign(&mut self, rhs: T) {
        *self += -rhs;
    }
}

impl<T: Ring, const N: usize> SubAssign for Monomial<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}
// poly * poly = poly
// poly * monomial = poly
// poly * T = poly (nie mozemy implementować dla dowolnego T)
// monomial * poly = poly
// monomial * monomial = monomial
// monomial * T = monomial (nie mozemy implementować dla dowolnego T)

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Mul for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.into_iter().fold(Polynomial::new(), |poly, m1| {
            rhs.iter()
                .fold(poly, |poly, m2| poly + (m1.clone() * m2.clone()))
        })
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Mul<Monomial<T, N>> for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn mul(mut self, rhs: Monomial<T, N>) -> Self::Output {
        if rhs.coef.is_zero() {
            self.root.take();
        }

        self.iter_mut().for_each(|m| m.coef *= rhs.coef.clone());

        self
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Mul<T> for Polynomial<T, N, O> {
    type Output = Polynomial<T, N, O>;

    fn mul(mut self, rhs: T) -> Self::Output {
        if rhs.is_zero() {
            self.root.take();
        }
        self.iter_mut().for_each(|m| m.coef *= rhs.clone());
        self
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Mul<Polynomial<T, N, O>> for Monomial<T, N> {
    type Output = Polynomial<T, N, O>;

    fn mul(self, mut rhs: Polynomial<T, N, O>) -> Self::Output {
        if self.coef.is_zero() {
            rhs.root.take();
        }

        rhs.iter_mut()
            .for_each(|m| m.coef = self.coef.clone() * m.coef.clone());

        rhs
    }
}

impl<T: Ring, const N: usize> Mul for Monomial<T, N> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.coef *= rhs.coef;
        (0..N).for_each(|i| self.powers[i] = rhs.powers[i]);
        self
    }
}

impl<T: Ring, const N: usize> Mul<T> for Monomial<T, N> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.coef *= rhs;
        self
    }
}

// polynomial *= poly
// polynomial *= monomial
// polynomial *= T
// mono *= mono
// mono *= T

impl<T: Ring, const N: usize, O: MonomialOrder<N>> MulAssign for Polynomial<T, N, O> {
    fn mul_assign(&mut self, rhs: Self) {
        if rhs.is_zero() {
            self.root.take();
        }

        let poly = Polynomial {
            root: self.root.take(),
            _o: std::marker::PhantomData::<O>,
        };

        self.root = (poly * rhs).root
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> MulAssign<Monomial<T, N>>
    for Polynomial<T, N, O>
{
    fn mul_assign(&mut self, rhs: Monomial<T, N>) {
        if rhs.coef.is_zero() {
            self.root.take();
        }

        self.iter_mut().for_each(|m| *m *= rhs.clone());
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> MulAssign<T> for Polynomial<T, N, O> {
    fn mul_assign(&mut self, rhs: T) {
        if rhs.is_zero() {
            self.root.take();
        }

        self.iter_mut().for_each(|m| m.coef *= rhs.clone());
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<T: Ring, const N: usize> MulAssign for Monomial<T, N> {
    fn mul_assign(&mut self, rhs: Self) {
        self.coef *= rhs.coef;
        (0..N).for_each(|i| self.powers[i] += rhs.powers[i]);
    }
}

impl<T: Ring, const N: usize> MulAssign<T> for Monomial<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        self.coef *= rhs;
    }
}

// monomial / monomial = monomial

impl<T: Field, const N: usize> Div for Monomial<T, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.coef.is_zero() {
            panic!("Division by 0")
        }

        let mut powers = [0; N];
        (0..N).for_each(|i| powers[i] = self.powers[i] - rhs.powers[i]);
        Self {
            coef: self.coef / rhs.coef,
            powers,
        }
    }
}

// monomia /= monomial

#[allow(clippy::suspicious_op_assign_impl)]
impl<T: Field, const N: usize> DivAssign for Monomial<T, N> {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.coef.is_zero() {
            panic!("Division by 0")
        }

        (0..N).for_each(|i| self.powers[i] -= rhs.powers[i]);
        self.coef /= rhs.coef;
    }
}
