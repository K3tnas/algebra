use std::{cmp::Ordering, marker::PhantomData};

use crate::{
    algstructs::{Monoid, Ring},
    polynomials::{
        monomial::Monomial,
        monomial_order::{Lex, MonomialOrder},
    },
};

pub mod monomial;
pub mod monomial_order;

pub mod single_var;
pub mod polynomial_reduce;
pub mod grobner_basis;

pub mod algstructimpl;
pub mod asfunc;
pub mod basict_traits;
pub mod iter;
pub mod ops;

#[derive(Clone, Default)]
pub struct Polynomial<T, const N: usize = 1, O = Lex<N>>
where
    T: Ring,
    O: MonomialOrder<N>,
{
    root: Option<Box<TreeNode<T, N>>>,
    _o: PhantomData<O>,
}

impl<T, O, const N: usize> Polynomial<T, N, O>
where
    T: Ring,
    O: MonomialOrder<N>,
{
    pub fn new() -> Self {
        Self {
            root: None,
            _o: PhantomData,
        }
    }

    pub fn lt(&self) -> Monomial<T, N> {
        if self.is_zero() {
            return Monomial {
                coef: T::zero(),
                powers: [0; N],
            };
        }

        let mut root = &self.root;
        while let Some(node) = root {
            if node.right.is_none() {
                return node.elem.clone();
            } else {
                root = &node.right
            }
        }

        Monomial {
            coef: T::zero(),
            powers: [0; N],
        }
    }

    pub(crate) fn insert(&mut self, elem: Monomial<T, N>) {
        if elem.coef == T::zero() {
            return;
        }

        self.root = Self::insert_node(self.root.take(), elem)
    }

    fn insert_node(
        node: Option<Box<TreeNode<T, N>>>,
        elem: Monomial<T, N>,
    ) -> Option<Box<TreeNode<T, N>>> {
        let Some(mut node) = node else {
            return Some(Box::new(TreeNode {
                left: None,
                right: None,
                elem,
            }));
        };

        match O::cmp(&node.elem, &elem) {
            Ordering::Less => node.left = Self::insert_node(node.left.take(), elem),
            Ordering::Greater => node.right = Self::insert_node(node.right.take(), elem),
            Ordering::Equal => {
                node.elem.coef = node.elem.coef.add(elem.coef);

                if node.elem.coef == T::zero() {
                    return Self::delete_node(node);
                }
            }
        }

        Some(node)
    }

    #[allow(clippy::boxed_local)]
    fn delete_node(mut node: Box<TreeNode<T, N>>) -> Option<Box<TreeNode<T, N>>> {
        match (node.left.take(), node.right.take()) {
            (None, None) => None,
            (None, right) => right,
            (left, None) => left,
            (Some(left), Some(right)) => {
                let (monomial, new_right) = Self::remove_min(right);
                Some(Box::new(TreeNode {
                    left: Some(left),
                    right: new_right,
                    elem: monomial,
                }))
            }
        }
    }

    fn remove_min(mut node: Box<TreeNode<T, N>>) -> (Monomial<T, N>, Option<Box<TreeNode<T, N>>>) {
        match node.left.take() {
            None => (node.elem, node.right.take()),
            Some(left) => {
                let (monomial, new_left) = Self::remove_min(left);
                node.left = new_left;
                (monomial, Some(node))
            }
        }
    }
}

impl<T, const N: usize, O1> Polynomial<T, N, O1>
where
    T: Ring,
    O1: MonomialOrder<N>,
{
    #[inline]
    pub fn reorder<O2: MonomialOrder<N>>(self) -> Polynomial<T, N, O2> {
        self.into_iter().collect()
    }
}

#[derive(Debug, Clone)]
struct TreeNode<T: Ring, const N: usize> {
    left: Option<Box<TreeNode<T, N>>>,
    right: Option<Box<TreeNode<T, N>>>,
    elem: Monomial<T, N>,
}
