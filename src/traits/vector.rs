use std::ops::{Add, Mul, AddAssign, MulAssign, Neg, Index, IndexMut};

use simba::scalar::{ComplexField, Field};
use num_traits::identities::Zero;

use super::{Matrix, MatrixMut};
// use pre::pre;

pub trait Vector
where
    for<'a> Self: Add<&'a Self> + Mul<Self::Elem> + Neg
{
    type Elem;

    fn zero_vec(dim: usize) -> Self;

    fn dim(&self) -> usize;

    fn get(&self, i: usize) -> Option<&Self::Elem>;

    /// # Ordinary vector norm squared
    fn norm_sqr(&self) -> Self::Elem
    where
        Self::Elem: Field + Clone
    {
        let mut acc = Self::Elem::zero();

        for i in 0..self.dim() {
            if let Some(val) = self.get(i) {
                acc += val.clone() * val.clone();
            }
        }

        acc
    }

    /// # Ordinary vector norm
    fn norm(&self) -> Self::Elem
    where
        Self::Elem: ComplexField + Clone
    {
        self.norm_sqr().sqrt()
    }

    /// # Complex vector norm squared
    fn cnorm_sqr(&self) -> <Self::Elem as ComplexField>::RealField
    where
        Self::Elem: ComplexField + Clone
    {
        let mut acc = <Self::Elem as ComplexField>::RealField::zero();

        for i in 0..self.dim() {
            if let Some(val) = self.get(i) {
                acc += val.clone().abs();
            }
        }

        acc
    }

    /// # Complex vector norm
    fn cnorm(&self) -> <Self::Elem as ComplexField>::RealField
    where
        Self::Elem: ComplexField + Clone
    {
        self.cnorm_sqr().sqrt()
    }

    /// # Euclidean bilinear dot product
    /// ## Preconditions
    /// A single non-panicing assertion, will truncate the product if not met:
    /// * `self` and `rhs` should have the same dim
    // #[pre("`self` and `rhs` should have the same dim")]
    fn dot_euclid<V>(&self, rhs: &V) -> Self::Elem
    where
        Self::Elem: Field + Clone,
        V: Vector<Elem = Self::Elem>
    {
        let mut acc = Self::Elem::zero();

        for i in 0..self.dim().min(rhs.dim()) {
            if let (Some(l), Some(r)) = (self.get(i), rhs.get(i)) {
                acc += l.clone() * r.clone();
            }
        }

        acc
    }

    /// # Euclidean sesquilinear dot product
    // #[pre("`self` and `rhs` should have the same dim")]
    fn cdot_euclid<V>(&self, rhs: &V) -> Self::Elem
    where
        Self::Elem: ComplexField + Clone,
        V: Vector<Elem = Self::Elem>
    {
        let mut acc = Self::Elem::zero();

        for i in 0..self.dim().min(rhs.dim()) {
            if let (Some(l), Some(r)) = (self.get(i), rhs.get(i)) {
                acc += l.clone().conjugate() * r.clone();
            }
        }

        acc
    }
}

pub trait VectorMut
where
    for<'a> Self: Vector + AddAssign<&'a Self> + MulAssign<Self::Elem>
{
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::Elem>;
}

impl<T, E> Matrix for T
where
    T: Vector<Elem = E>,
{
    type Elem = E;

    fn zero_mat(rows: usize, _cols: usize) -> Self {
        Self::zero_vec(rows)
    }

    fn nrows(&self) -> usize {
        Self::dim(self)
    }

    fn ncols(&self) -> usize {
        1
    }

    fn get(&self, i: usize, j: usize) -> Option<&Self::Elem> {
        if j != 0 {
            None
        } else {
            Self::get(self, i)
        }
    }
}

impl<T, E> MatrixMut for T
where
    T: VectorMut<Elem = E>
{
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Elem> {
        if j != 0 {
            None
        } else {
            Self::get_mut(self, i)
        }
    }
}

pub trait FullVector: Vector + Index<usize, Output = Self::Elem> { }
impl<T> FullVector for T
where
    T: Vector + Index<usize, Output = Self::Elem>
{

}

pub trait FullVectorMut: FullVector + VectorMut + IndexMut<usize> { }
impl<T> FullVectorMut for T
where
    T: FullVector + VectorMut + IndexMut<usize>
{

}
