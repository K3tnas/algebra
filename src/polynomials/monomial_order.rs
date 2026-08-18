use std::{cmp::Ordering, marker::PhantomData};

use crate::{algstructs::Ring, polynomials::monomial::Monomial};

pub trait MonomialOrder<const N: usize>: Clone {
    fn cmp<A: Ring, B: Ring>(a: &Monomial<A, N>, b: &Monomial<B, N>) -> std::cmp::Ordering;
}

#[derive(Clone)]
pub struct Lex<const N: usize, P: Perm<N> = DefaultPerm> {
    _p: PhantomData<P>,
}

#[derive(Clone)]
pub struct GradedLex<const N: usize, P: Perm<N> = DefaultPerm> {
    _p: PhantomData<P>,
}

impl<const N: usize, P: Perm<N>> MonomialOrder<N> for Lex<N, P> {
    fn cmp<A: Ring, B: Ring>(a: &Monomial<A, N>, b: &Monomial<B, N>) -> std::cmp::Ordering {
        for i in 0..N {
            let j = P::PERM[i];
            match a.powers[j].cmp(&b.powers[j]) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

impl<const N: usize, P: Perm<N>> MonomialOrder<N> for GradedLex<N, P> {
    fn cmp<A: Ring, B: Ring>(a: &Monomial<A, N>, b: &Monomial<B, N>) -> std::cmp::Ordering {
        match a.powers.iter().sum::<u32>().cmp(&b.powers.iter().sum()) {
            Ordering::Equal => (),
            ord => return ord,
        }

        for i in 0..N {
            let j = P::PERM[i];
            match a.powers[j].cmp(&b.powers[j]) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

pub trait Perm<const N: usize>: Clone {
    const PERM: [usize; N];
}

#[derive(Clone)]
pub struct DefaultPerm;

impl<const N: usize> Perm<N> for DefaultPerm {
    const PERM: [usize; N] = {
        let mut arr = [0usize; N];
        let mut i = 0;
        while i < N {
            arr[i] = i;
            i += 1;
        }
        arr
    };
}

#[doc(hidden)]
pub mod __private {
    use crate::polynomials::monomial_order::Perm;

    #[derive(Clone, Copy)]
    pub struct EncodedPerm<const N: usize, const CODE: u64>;

    impl<const N: usize, const CODE: u64> Perm<N> for EncodedPerm<N, CODE> {
        const PERM: [usize; N] = decode_perm::<N>(CODE);
    }

    pub const fn encode_perm<const N: usize>(p: [usize; N]) -> u64 {
        let mut code: u64 = 0;
        let mut i = N;
        while i > 0 {
            i -= 1;
            code = code * N as u64 + p[i] as u64;
        }
        code
    }

    const fn decode_perm<const N: usize>(mut code: u64) -> [usize; N] {
        let mut arr = [0usize; N];
        let mut i = 0;
        while i < N {
            arr[i] = (code % N as u64) as usize;
            code /= N as u64;
            i += 1;
        }
        arr
    }
}

#[macro_export]
macro_rules! lex {
    ($($p:expr),+ $(,)?) => {
        $crate::polynomials::monomial_order::Lex<
            { [$($p),+].len() },
            $crate::polynomials::monomial_order::__private::EncodedPerm<
                { [$($p),+].len() },
                { $crate::polynomials::monomial_order::__private::encode_perm([$($p),+]) }
            >
        >
    };
}

#[macro_export]
macro_rules! gradedlex {
    ($($p:expr),+ $(,)?) => {
        $crate::polynomials::monomial_order::GradedLex<
            { [$($p),+].len() },
            $crate::polynomials::monomial_order::__private::EncodedPerm<
                { [$($p),+].len() },
                { $crate::polynomials::monomial_order::__private::encode_perm([$($p),+]) }
            >
        >
    };
}
