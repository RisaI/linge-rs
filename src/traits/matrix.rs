use std::ops::{Add, Mul, AddAssign, MulAssign, Neg, Index, IndexMut};

use num_traits::Zero;

use super::Vector;

pub trait Matrix
where
    for<'a> Self: Add<&'a Self> + Mul<Self::Elem> + Neg
{
    type Elem;

    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;

    fn get(&self, i: usize, j: usize) -> Option<&Self::Elem>;

    fn is_zero(&self) -> bool
    where
        Self::Elem: Zero
    {
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                if let Some(r) = self.get(i, j) {
                    if !r.is_zero() {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

pub trait MatrixMut
where
    for<'a> Self: Matrix + AddAssign<&'a Self> + MulAssign<Self::Elem>
{
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Elem>;

    fn set_zero(&mut self)
    where
        Self::Elem: Zero
    {
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                if let Some(r) = self.get_mut(i, j) {
                    r.set_zero();
                }
            }
        }
    }
}

pub trait FullMatrix: Matrix + Index<(usize, usize), Output = Self::Elem> { }
impl<T> FullMatrix for T
where
    T: Matrix + Index<(usize, usize), Output = Self::Elem>
{

}

pub trait FullMatrixMut: FullMatrix + MatrixMut + IndexMut<(usize, usize)> { }
impl<T> FullMatrixMut for T
where
    T: FullMatrix + MatrixMut + IndexMut<(usize, usize)>
{

}

pub trait MatVecMul<Rhs: Vector>
where
    for<'a,'b> &'a Self: Mul<&'b Rhs, Output = Self::Output>
{
    type Output: Vector;
}
