use std::ops::{Index, IndexMut, Neg, Mul, MulAssign, Add, AddAssign};

use crate::{traits::{Vector, VectorMut}, Field};

/// # Static Vector<T>
#[derive(Clone)]
pub struct SVector<T, const DIM: usize>([T; DIM]);

impl<T, const DIM: usize> Index<usize> for SVector<T, DIM> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const DIM: usize> IndexMut<usize> for SVector<T, DIM> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Field + Clone, const DIM: usize> Neg for SVector<T, DIM> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.0.iter_mut()
            .for_each(|v| {
                *v = v.clone().neg();
            });

        self
    }
}

impl<T: Field + Clone, const DIM: usize> MulAssign<T> for SVector<T, DIM> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.iter_mut()
            .for_each(|v| {
                *v *= rhs.clone();
            });
    }
}

impl<T: Field + Clone, const DIM: usize> Mul<T> for SVector<T, DIM> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;

        self
    }
}

impl<'a, T: Field + Clone, const DIM: usize> AddAssign<&'a SVector<T, DIM>> for SVector<T, DIM> {
    fn add_assign(&mut self, rhs: &'a SVector<T, DIM>) {
        self.0.iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(l, r)| {
                *l += r.clone();
            });
    }
}

impl<'a, T: Field + Clone, const DIM: usize> Add<&'a SVector<T, DIM>> for SVector<T, DIM> {
    type Output = Self;

    fn add(mut self, rhs: &'a SVector<T, DIM>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Field + Copy + Default, const DIM: usize> Vector for SVector<T, DIM> {
    type Elem = T;

    fn zero_vec(dim: usize) -> Self {
        // ! this must go
        assert_eq!(dim, DIM);
        Self([T::default(); DIM])
    }

    fn dim(&self) -> usize {
        DIM
    }

    fn get(&self, i: usize) -> Option<&Self::Elem> {
        if i >= DIM {
            None
        } else {
            Some(&self[i])
        }
    }
}

impl<T: Field + Copy + Default, const DIM: usize> VectorMut for SVector<T, DIM> {
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::Elem> {
        if i >= DIM {
            None
        } else {
            Some(&mut self.0[i])
        }
    }
}
