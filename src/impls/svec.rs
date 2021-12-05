use std::ops::{Index, IndexMut, Neg, Mul, MulAssign, Add, AddAssign};

use crate::{traits::{Vector, VectorMut}, Field};

use super::DVector;

/// # Static Vector<T>
#[derive(Clone)]
pub struct SVector<T, const DIM: usize>([T; DIM]);

// SECTION std::ops impls

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

// !SECTION

// SECTION Vector impls

impl<T: Field + Clone, const DIM: usize> Vector for SVector<T, DIM> {
    type Elem = T;

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

impl<T: Field + Clone, const DIM: usize> VectorMut for SVector<T, DIM> {
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::Elem> {
        if i >= DIM {
            None
        } else {
            Some(&mut self.0[i])
        }
    }
}

// !SECTION

impl<T: Field, const DIM: usize> Add<SVector<T, DIM>> for SVector<T, DIM> {
    type Output = Self;

    fn add(mut self, rhs: SVector<T, DIM>) -> Self::Output {
        self.0.iter_mut()
            .zip(rhs.0.into_iter())
            .for_each(|(t, v)| {
                *t += v;
            });

        self
    }
}

impl<T: Field, const DIM: usize> num_traits::Zero for SVector<T, DIM> {
    fn zero() -> Self {
        Self([(); DIM].map(|_| T::zero()))
    }

    fn is_zero(&self) -> bool {
        !self.0.iter().any(|e| !e.is_zero())
    }
}

// ANCHOR From impls

impl<T: Field, F: Into<[T; DIM]>, const DIM: usize> From<F> for SVector<T, DIM> {
    fn from(b: F) -> Self {
        Self(b.into())
    }
}

// ANCHOR custom impls

impl<T: Field, const DIM: usize> Into<DVector<T>> for SVector<T, DIM> {
    fn into(self) -> DVector<T> {
        self.0.into()
    }
}
