use crate::{
    algstructs::{Field, Monoid},
    polynomials::{Polynomial, monomial::Monomial, monomial_order::MonomialOrder},
};

impl<T, const N: usize, O> Polynomial<T, N, O>
where
    T: Field,
    O: MonomialOrder<N>,
{
    pub fn grobner_basis(g: &[Self]) -> Vec<Self> {
        let n = g.len();
        let mut g = g.to_vec();
        let mut queue = Vec::<(Self, Self)>::with_capacity(n * n);

        for i in 0..n {
            for j in i + 1..n {
                queue.push((g[i].clone(), g[j].clone()));
            }
        }

        while let Some((w1, w2)) = queue.pop() {
            let r = w1.syzygy(w2).reduce(&g).1;

            if r.is_zero() {
                continue;
            }

            g.iter().for_each(|f| queue.push((f.clone(), r.clone())));
            g.push(r);
        }

        g
    }

    pub fn reduce_basis(mut g: Vec<Self>) -> Vec<Self> {
        let mut i = 0;
        while i < g.len() {
            let lt_i = g[i].lt();
            let redundant = g
                .iter()
                .enumerate()
                .any(|(j, f)| j != i && f.lt().divides(&lt_i));
            if redundant {
                g.remove(i);
            } else {
                i += 1;
            }
        }

        for i in 0..g.len() {
            let others: Vec<_> = g
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, f)| f.clone())
                .collect();
            g[i] = g[i].clone().reduce(&others).1;
        }

        g
    }

    fn syzygy(self, other: Self) -> Self {
        let lt_f = self.lt();
        let lt_g = other.lt();

        let l = lt_f.lcm(&lt_g);

        (self * (l.clone() / lt_f)) - (other * (l / lt_g))
    }
}

impl<T, const N: usize> Monomial<T, N>
where
    T: Field,
{
    fn lcm(&self, other: &Self) -> Self {
        let mut powers = [0; N];
        (0..N).for_each(|i| powers[i] = self.powers[i].max(other.powers[i]));
        Self {
            coef: T::one(),
            powers,
        }
    }
}
