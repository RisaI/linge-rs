use std::ops::{Add, Mul, AddAssign, MulAssign, Neg, Index, IndexMut};

pub trait Matrix
where
    for<'a> Self: Add<&'a Self> + Mul<Self::Elem> + Neg
{
    type Elem;

    fn zero_mat(rows: usize, cols: usize) -> Self;

    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;

    fn get(&self, i: usize, j: usize) -> Option<&Self::Elem>;
}

pub trait MatrixMut
where
    for<'a> Self: Matrix + AddAssign<&'a Self> + MulAssign<Self::Elem>
{
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Elem>;
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
