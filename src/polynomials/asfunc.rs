use crate::{
    algstructs::Ring,
    polynomials::{Polynomial, monomial_order::MonomialOrder},
};

impl<T: Ring, O: MonomialOrder<1>> FnOnce<(T,)> for Polynomial<T, 1, O> {
    type Output = T;

    extern "rust-call" fn call_once(self, args: (T,)) -> Self::Output {
        let (x,) = args;
        self.into_iter().fold(T::zero(), |sum, m| {
            sum + m.coef * (x.clone().pow(m.powers[0]))
        })
    }
}

impl<T: Ring, O: MonomialOrder<1>> FnMut<(T,)> for Polynomial<T, 1, O> {
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> Self::Output {
        let (x,) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone() * (x.clone().pow(m.powers[0]))
        })
    }
}

impl<T: Ring, O: MonomialOrder<1>> Fn<(T,)> for Polynomial<T, 1, O> {
    extern "rust-call" fn call(&self, args: (T,)) -> Self::Output {
        let (x,) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone() * (x.clone().pow(m.powers[0]))
        })
    }
}

impl<T: Ring, O: MonomialOrder<2>> FnOnce<(T, T)> for Polynomial<T, 2, O> {
    type Output = T;
    extern "rust-call" fn call_once(self, args: (T, T)) -> Self::Output {
        let (x, y) = args;
        self.into_iter().fold(T::zero(), |sum, m| {
            sum + m.coef * x.clone().pow(m.powers[0]) * y.clone().pow(m.powers[1])
        })
    }
}

impl<T: Ring, O: MonomialOrder<2>> FnMut<(T, T)> for Polynomial<T, 2, O> {
    extern "rust-call" fn call_mut(&mut self, args: (T, T)) -> Self::Output {
        let (x, y) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone() * x.clone().pow(m.powers[0]) * y.clone().pow(m.powers[1])
        })
    }
}

impl<T: Ring, O: MonomialOrder<2>> Fn<(T, T)> for Polynomial<T, 2, O> {
    extern "rust-call" fn call(&self, args: (T, T)) -> Self::Output {
        let (x, y) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone() * x.clone().pow(m.powers[0]) * y.clone().pow(m.powers[1])
        })
    }
}

impl<T: Ring, O: MonomialOrder<3>> FnOnce<(T, T, T)> for Polynomial<T, 3, O> {
    type Output = T;
    extern "rust-call" fn call_once(self, args: (T, T, T)) -> Self::Output {
        let (x, y, z) = args;
        self.into_iter().fold(T::zero(), |sum, m| {
            sum + m.coef
                * x.clone().pow(m.powers[0])
                * y.clone().pow(m.powers[1])
                * z.clone().pow(m.powers[2])
        })
    }
}

impl<T: Ring, O: MonomialOrder<3>> FnMut<(T, T, T)> for Polynomial<T, 3, O> {
    extern "rust-call" fn call_mut(&mut self, args: (T, T, T)) -> Self::Output {
        let (x, y, z) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone()
                * x.clone().pow(m.powers[0])
                * y.clone().pow(m.powers[1])
                * z.clone().pow(m.powers[2])
        })
    }
}

impl<T: Ring, O: MonomialOrder<3>> Fn<(T, T, T)> for Polynomial<T, 3, O> {
    extern "rust-call" fn call(&self, args: (T, T, T)) -> Self::Output {
        let (x, y, z) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone()
                * x.clone().pow(m.powers[0])
                * y.clone().pow(m.powers[1])
                * z.clone().pow(m.powers[2])
        })
    }
}

fn eval_monomial_powers<T: Ring, const N: usize>(x: &[T; N], powers: &[u32; N]) -> T {
    let mut acc = T::one();
    for i in 0..N {
        acc *= x[i].clone().pow(powers[i]);
    }
    acc
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> FnOnce<([T; N],)> for Polynomial<T, N, O> {
    type Output = T;
    extern "rust-call" fn call_once(self, args: ([T; N],)) -> Self::Output {
        let (x,) = args;
        self.into_iter().fold(T::zero(), |sum, m| {
            sum + m.coef * eval_monomial_powers(&x, &m.powers)
        })
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> FnMut<([T; N],)> for Polynomial<T, N, O> {
    extern "rust-call" fn call_mut(&mut self, args: ([T; N],)) -> Self::Output {
        let (x,) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone() * eval_monomial_powers(&x, &m.powers)
        })
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Fn<([T; N],)> for Polynomial<T, N, O> {
    extern "rust-call" fn call(&self, args: ([T; N],)) -> Self::Output {
        let (x,) = args;
        self.iter().fold(T::zero(), |sum, m| {
            sum + m.coef.clone() * eval_monomial_powers(&x, &m.powers)
        })
    }
}
