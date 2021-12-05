use std::ops::{Index, IndexMut, Neg, Mul, MulAssign, Add, AddAssign};

use replace_with::replace_with_or_abort;

use crate::{traits::{Vector, VectorMut}, Field};

/// # Dynamic Vector<T>
/// A dynamically sized vector with arbitrary element type
#[derive(Clone)]
pub struct DVector<T>(Box<[T]>);

impl<T> Index<usize> for DVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for DVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Field> Neg for DVector<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.0.iter_mut()
            .for_each(|v| {
                replace_with_or_abort(v, |val| val.neg());
            });

        self
    }
}

impl<T: Field + Clone> MulAssign<T> for DVector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.iter_mut()
            .for_each(|v| {
                *v *= rhs.clone();
            });
    }
}

impl<T: Field + Clone> Mul<T> for DVector<T> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;

        self
    }
}

impl<'a, T: Field + Clone> AddAssign<&'a DVector<T>> for DVector<T> {
    fn add_assign(&mut self, rhs: &'a DVector<T>) {
        self.0.iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(l, r)| {
                *l += r.clone();
            });
    }
}

impl<'a, T: Field + Clone> Add<&'a DVector<T>> for DVector<T> {
    type Output = Self;

    fn add(mut self, rhs: &'a DVector<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Field + Clone> Vector for DVector<T> {
    type Elem = T;

    fn dim(&self) -> usize {
        self.0.len()
    }

    fn get(&self, i: usize) -> Option<&Self::Elem> {
        if i >= self.dim() {
            None
        } else {
            Some(&self[i])
        }
    }
}

impl<T: Field + Clone> VectorMut for DVector<T> {
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::Elem> {
        if i >= self.dim() {
            None
        } else {
            Some(&mut self.0[i])
        }
    }
}

// From Impls
impl<T, F: Into<Box<[T]>>> From<F> for DVector<T> {
    fn from(b: F) -> Self {
        Self(b.into())
    }
}

impl<T> FromIterator<T> for DVector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
