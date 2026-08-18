use std::{
    iter::{Product, Sum},
    marker::PhantomData,
};

use crate::{
    algstructs::Ring,
    polynomials::{Polynomial, TreeNode, monomial::Monomial, monomial_order::MonomialOrder},
};

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Polynomial<T, N, O> {
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter::new(&self.root)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut::new(&mut self.root)
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> IntoIterator for Polynomial<T, N, O> {
    type Item = Monomial<T, N>;

    type IntoIter = Monomials<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Monomials::new(self.root)
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Sum<Monomial<T, N>> for Polynomial<T, N, O> {
    fn sum<I: Iterator<Item = Monomial<T, N>>>(iter: I) -> Self {
        iter.fold(Polynomial::new(), |mut poly, monomial| {
            poly.insert(monomial);
            poly
        })
    }
}

impl<T: Ring, const N: usize> Product for Monomial<T, N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Monomial {
                coef: T::one(),
                powers: [0; N],
            },
            |mut product, next| {
                product.coef = product.coef.mul(next.coef);
                (0..N).for_each(|i| product.powers[i] += next.powers[i]);
                product
            },
        )
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Product<Monomial<T, N>> for Polynomial<T, N, O> {
    fn product<I: Iterator<Item = Monomial<T, N>>>(iter: I) -> Self {
        let mut poly = Polynomial::new();
        poly.insert(iter.product::<Monomial<T, N>>());
        poly
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Sum for Polynomial<T, N, O> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Polynomial::new(), |base, p| base + p)
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> Product for Polynomial<T, N, O> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Polynomial::new(), |base, p| base * p)
    }
}

impl<T: Ring, const N: usize, O: MonomialOrder<N>> FromIterator<Monomial<T, N>>
    for Polynomial<T, N, O>
{
    fn from_iter<I: IntoIterator<Item = Monomial<T, N>>>(iter: I) -> Self {
        iter.into_iter().fold(Polynomial::new(), |p, m| p + m)
    }
}

pub struct Iter<'a, T: Ring, const N: usize> {
    stack: Vec<&'a TreeNode<T, N>>,
}

pub struct Monomials<T: Ring, const N: usize> {
    stack: Vec<Box<TreeNode<T, N>>>,
}

pub struct IterMut<'a, T: Ring, const N: usize> {
    stack: Vec<*mut TreeNode<T, N>>,
    _m: PhantomData<&'a mut TreeNode<T, N>>,
}

impl<'a, T: Ring, const N: usize> Iter<'a, T, N> {
    fn new(root: &'a Option<Box<TreeNode<T, N>>>) -> Self {
        let mut stack = Vec::new();
        push_right_spine(root.as_deref(), &mut stack);
        Self { stack }
    }
}

fn push_right_spine<'a, T: Ring, const N: usize>(
    mut node: Option<&'a TreeNode<T, N>>,
    stack: &mut Vec<&'a TreeNode<T, N>>,
) {
    while let Some(n) = node {
        stack.push(n);
        node = n.right.as_deref();
    }
}

impl<'a, T: Ring, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a Monomial<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        push_right_spine(node.left.as_deref(), &mut self.stack);
        Some(&node.elem)
    }
}

impl<T: Ring, const N: usize> Monomials<T, N> {
    fn new(root: Option<Box<TreeNode<T, N>>>) -> Self {
        let mut stack = Vec::new();
        push_right_spine_owned(root, &mut stack);
        Self { stack }
    }
}

fn push_right_spine_owned<T: Ring, const N: usize>(
    mut node: Option<Box<TreeNode<T, N>>>,
    stack: &mut Vec<Box<TreeNode<T, N>>>,
) {
    while let Some(mut n) = node {
        node = n.right.take();
        stack.push(n);
    }
}

impl<T: Ring, const N: usize> Iterator for Monomials<T, N> {
    type Item = Monomial<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop()?;
        push_right_spine_owned(node.left.take(), &mut self.stack);
        Some(node.elem)
    }
}

impl<'a, T: Ring, const N: usize> IterMut<'a, T, N> {
    fn new(root: &mut Option<Box<TreeNode<T, N>>>) -> Self {
        let mut stack = Vec::new();
        push_right_spine_mut(root.as_deref_mut(), &mut stack);
        Self {
            stack,
            _m: PhantomData,
        }
    }
}

fn push_right_spine_mut<T: Ring, const N: usize>(
    mut node: Option<&mut TreeNode<T, N>>,
    stack: &mut Vec<*mut TreeNode<T, N>>,
) {
    while let Some(n) = node {
        let ptr = n as *mut TreeNode<T, N>;
        node = n.right.as_deref_mut();
        stack.push(ptr);
    }
}

impl<'a, T: Ring, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut Monomial<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.stack.pop()?;
        let node = unsafe { &mut *ptr };
        push_right_spine_mut(node.left.as_deref_mut(), &mut self.stack);
        Some(&mut node.elem)
    }
}
