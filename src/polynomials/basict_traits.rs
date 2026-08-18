use std::fmt::{self, Display, Formatter};

use crate::{
    algstructs::{Monoid, Ring},
    polynomials::{Polynomial, monomial::Monomial, monomial_order::MonomialOrder},
};

impl<T: Ring, const N: usize, O: MonomialOrder<N>> PartialEq for Polynomial<T, N, O> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, const N: usize> Monomial<T, N>
where
    T: Ring + Display,
{
    pub fn display_with<'a>(&'a self, names: &'a [&'a str; N]) -> MonomialDisplay<'a, T, N> {
        MonomialDisplay {
            monomial: self,
            names,
        }
    }
}

impl<T, const N: usize, O> Polynomial<T, N, O>
where
    T: Ring + Display,
    O: MonomialOrder<N>,
{
    pub fn display_with<'a>(&'a self, names: &'a [&'a str; N]) -> PolynomialDisplay<'a, T, N, O> {
        PolynomialDisplay {
            polynomial: self,
            names,
        }
    }
}

pub struct MonomialDisplay<'a, T, const N: usize>
where
    T: Ring + Display,
{
    monomial: &'a Monomial<T, N>,
    names: &'a [&'a str; N],
}

pub struct PolynomialDisplay<'a, T, const N: usize, O>
where
    T: Ring + Display,
    O: MonomialOrder<N>,
{
    polynomial: &'a Polynomial<T, N, O>,
    names: &'a [&'a str; N],
}

impl<T, const N: usize> Display for MonomialDisplay<'_, T, N>
where
    T: Ring + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match &self.monomial.coef {
                coef if coef.is_one() && !self.monomial.powers.iter().all(|pow| *pow == 0) =>
                    "".to_string(),
                coef => format!("{{{}}}", coef),
            },
            (0..N)
                .filter(|i| self.monomial.powers[*i] > 0)
                .map(|i| {
                    let mut s = String::from(self.names[i]);
                    let pow = self.monomial.powers[i];
                    if pow > 1 {
                        s += &format!("^{pow}");
                    }

                    s
                })
                .collect::<String>()
        )
    }
}

impl<T, const N: usize, O> Display for PolynomialDisplay<'_, T, N, O>
where
    T: Ring + Display,
    O: MonomialOrder<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.polynomial.is_zero() {
            return write!(f, "{}", T::zero());
        }

        write!(
            f,
            "{}",
            self.polynomial
                .iter()
                .map(|m| m.display_with(self.names).to_string())
                .collect::<Vec<String>>()
                .into_iter()
                .rev()
                .collect::<Vec<String>>()
                .join(" + ")
        )
    }
}
