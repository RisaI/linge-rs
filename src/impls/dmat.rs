use std::ops::{Index, IndexMut, Neg, Mul, MulAssign, Add, AddAssign};

use replace_with::replace_with_or_abort;
use simba::scalar::Field;

use crate::prelude::{Matrix, MatrixMut};

#[derive(Clone)]
pub struct DMatrix<T>(usize /* ncols */, Box<[T]>);

impl<T> Index<(usize, usize)> for DMatrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.1[i * self.0 + j]
    }
}

impl<T> IndexMut<(usize, usize)> for DMatrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.1[i * self.0 + j]
    }
}

impl<T: Field> Neg for DMatrix<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.1.iter_mut()
            .for_each(|v| {
                replace_with_or_abort(v, |val| val.neg());
            });

        self
    }
}

impl<T: Field + Clone> MulAssign<T> for DMatrix<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.1.iter_mut()
            .for_each(|v| {
                *v *= rhs.clone();
            });
    }
}

impl<T: Field + Clone> Mul<T> for DMatrix<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;

        self
    }
}
impl<'a, T: Field + Clone> AddAssign<&'a DMatrix<T>> for DMatrix<T> {
    fn add_assign(&mut self, rhs: &'a DMatrix<T>) {
        // REVIEW make a better misaligned size handling

        self.1.iter_mut()
            .zip(rhs.1.iter())
            .for_each(|(l, r)| {
                *l += r.clone();
            });
    }
}

impl<'a, T: Field + Clone> Add<&'a DMatrix<T>> for DMatrix<T> {
    type Output = Self;

    fn add(mut self, rhs: &'a DMatrix<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Field + Clone> Matrix for DMatrix<T> {
    type Elem = T;

    fn nrows(&self) -> usize {
        self.1.len() / self.0
    }

    fn ncols(&self) -> usize {
        self.0
    }

    fn get(&self, i: usize, j: usize) -> Option<&Self::Elem> {
        if i >= self.nrows() || j >= self.ncols() {
            None
        } else {
            Some(&self[(i, j)])
        }
    }
}

impl<T: Field + Clone> MatrixMut for DMatrix<T> {
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Elem> {
        if i >= self.nrows() || j >= self.ncols() {
            None
        } else {
            Some(&mut self[(i, j)])
        }
    }
}
