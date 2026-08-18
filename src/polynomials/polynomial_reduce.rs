use crate::{
    algstructs::{Field, Monoid}, polynomials::{Polynomial, monomial::Monomial, monomial_order::MonomialOrder},
};

impl<T: Field, const N: usize, O: MonomialOrder<N>> Polynomial<T, N, O> {
    pub fn reduce(mut self, gs: &[Self]) -> (Vec<Self>, Self) {
        let mut alphas = vec![Polynomial::new(); gs.len()];
        let mut r = Polynomial::new();

        while !self.is_zero() {
            let mut divided = false;
            for (i, g) in gs.iter().enumerate() {
                let (lt_g, lt_f) = (g.lt(), self.lt());
                if divides(&lt_g, &lt_f) {
                    let q = lt_f / lt_g;

                    alphas[i] += q.clone();
                    self -= q * g.clone();

                    divided = true;
                    break;
                }
            }

            if !divided {
                let lt_f = self.lt();
                r += lt_f.clone();
                self -= lt_f;
            }
        }

        (alphas, r)
    }
}

fn divides<T: Field, const N: usize>(a: &Monomial<T, N>, b: &Monomial<T, N>) -> bool {
    if a.coef.is_zero() {
        return false;
    }

    for i in 0..N {
        if b.powers[i] < a.powers[i] {
            return false;
        }
    }

    true
}
