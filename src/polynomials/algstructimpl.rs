use std::marker::PhantomData;

use crate::{
    algstructs::{Group, Monoid, Ring},
    polynomials::{Polynomial, TreeNode, monomial::Monomial, monomial_order::MonomialOrder},
};

impl<T, const N: usize, O> Monoid for Polynomial<T, N, O>
where
    T: Ring,
    O: MonomialOrder<N>,
{
    fn zero() -> Self {
        Polynomial {
            root: None,
            _o: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.root.is_none()
    }
}

impl<T, const N: usize, O> Group for Polynomial<T, N, O>
where
    T: Ring,
    O: MonomialOrder<N>,
{
}

impl<T, const N: usize, O> Ring for Polynomial<T, N, O>
where
    T: Ring,
    O: MonomialOrder<N>,
{
    fn one() -> Self {
        Polynomial {
            root: Some(Box::new(TreeNode {
                left: None,
                right: None,
                elem: Monomial {
                    coef: T::one(),
                    powers: [0; N],
                },
            })),
            _o: PhantomData,
        }
    }

    fn is_one(&self) -> bool {
        let Some(node) = &self.root else { return false };
        matches!(
            (&node.left, &node.right),
            (None, None) if node.elem.coef.is_one()
                && node.elem.powers.iter().all(|&pow| pow == 0)
        )
    }
}
