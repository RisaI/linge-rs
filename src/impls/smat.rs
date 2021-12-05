use std::ops::{Index, IndexMut, Neg, Mul, MulAssign, Add, AddAssign};

use crate::{traits::{Matrix, MatrixMut}, Field};

use num_traits::Zero;
use replace_with::replace_with_or_abort;

use super::SVector;

/// # Static Matrix<T, ROWS, COLS>
// #[derive(Clone)]
pub struct SMatrix<T: Sized, const ROWS: usize, const COLS: usize>([SVector<T, ROWS>; COLS]);

// SECTION std::ops impls

impl<T: Sized, const ROWS: usize, const COLS: usize> Index<(usize, usize)> for SMatrix<T, ROWS, COLS> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.0[i][j]
    }
}

impl<T: Sized, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)> for SMatrix<T, ROWS, COLS> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i][j]
    }
}

impl<T: Field + Clone + Sized, const ROWS: usize, const COLS: usize> Neg for SMatrix<T, ROWS, COLS> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.0.iter_mut()
            .for_each(|col| {
                replace_with_or_abort(col, |val| val.neg());
            });

        self
    }
}

impl<T: Field + Clone, const ROWS: usize, const COLS: usize> MulAssign<T> for SMatrix<T, ROWS, COLS> {
    fn mul_assign(&mut self, rhs: T) {
        self.0.iter_mut()
            .for_each(|col| {
                *col *= rhs.clone();
            });
    }
}

impl<T: Field + Clone, const ROWS: usize, const COLS: usize> Mul<T> for SMatrix<T, ROWS, COLS> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;

        self
    }
}

impl<'a, T: Field + Clone, const ROWS: usize, const COLS: usize> AddAssign<&'a SMatrix<T, ROWS, COLS>> for SMatrix<T, ROWS, COLS> {
    fn add_assign(&mut self, rhs: &'a SMatrix<T, ROWS, COLS>) {
        self.0.iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(lcol, rcol)| {
                *lcol += rcol;
            });
    }
}

impl<'a, T: Field + Clone, const ROWS: usize, const COLS: usize> Add<&'a SMatrix<T, ROWS, COLS>> for SMatrix<T, ROWS, COLS> {
    type Output = Self;

    fn add(mut self, rhs: &'a SMatrix<T, ROWS, COLS>) -> Self::Output {
        self += rhs;
        self
    }
}

// !SECTION

// SECTION Matrix impls

impl<T: Field + Clone + Sized, const ROWS: usize, const COLS: usize> Matrix for SMatrix<T, ROWS, COLS> {
    type Elem = T;

    fn nrows(&self) -> usize {
        ROWS
    }

    fn ncols(&self) -> usize {
        COLS
    }

    fn get(&self, i: usize, j: usize) -> Option<&Self::Elem> {
        if i >= ROWS && j >= COLS {
            None
        } else {
            Some(&self[(i, j)])
        }
    }
}

impl<T: Field + Clone + Sized, const ROWS: usize, const COLS: usize> MatrixMut for SMatrix<T, ROWS, COLS> {
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Elem> {
        if i >= ROWS && j >= COLS {
            None
        } else {
            Some(&mut self[(i, j)])
        }
    }
}

// !SECTION

// ANCHOR From impls

impl<T: Field + Sized, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for SMatrix<T, ROWS, COLS> {
    fn from(b: [[T; COLS]; ROWS]) -> Self {
        let mut result = SMatrix([(); COLS].map(|_| SVector::zero()));

        b.into_iter()
            .flatten()
            .enumerate()
            .for_each(|(idx, val)| {
                result[(idx / COLS, idx % COLS)] = val;
            });
        
        result
    }
}

impl<T: Field + Sized, const ROWS: usize, const COLS: usize> From<[SVector<T, ROWS>; COLS]> for SMatrix<T, ROWS, COLS> {
    fn from(cols: [SVector<T, ROWS>; COLS]) -> Self {
        Self(
            cols
        )
    }
}

// ANCHOR custom impls

// impl<T: Field, const ROWS: usize, const COLS: usize> Into<DVector<T>> for SMatrix<T, ROWS, COLS> {
//     fn into(self) -> DVector<T> {
//         self.0.into()
//     }
// }

impl<T: Field + Clone + Sized, const ROWS: usize> SMatrix<T, ROWS, 1> {
    pub fn into_svector(self) -> SVector<T, ROWS> {
        match self.0 {
            [ vec ] => vec
        }
    }
}

impl<T: Clone + Sized, const ROWS: usize, const COLS: usize> SMatrix<T, ROWS, COLS> {
    pub fn clone_col(&self, col: usize) -> Option<SVector<T, ROWS>> {
        if col >= COLS {
            None
        } else {
            Some(self.0[col].clone())
        }
    }

    pub fn get_col(&self, col: usize) ->  Option<&SVector<T, ROWS>> {
        if col >= COLS {
            None
        } else {
            Some(&self.0[col])
        }
    }
}

impl<'a, 'b, T: Field + Clone, const ROWS: usize, const COLS: usize, const TCOLS: usize> Mul<&'a SMatrix<T, COLS, TCOLS>> for &'b SMatrix<T, ROWS, COLS> {
    type Output = SMatrix<T, ROWS, TCOLS>;

    fn mul(self, rhs: &'a SMatrix<T, COLS, TCOLS>) -> Self::Output {
        let mut col: usize = 0;

        [(); TCOLS]
            .map(|_| {
                col += 1;
                self * rhs.get_col(col - 1).unwrap()
            })
            .into()
    }
}

impl<'a, 'b, T: Field + Clone, const ROWS: usize, const COLS: usize> Mul<&'a SVector<T, COLS>> for &'b SMatrix<T, ROWS, COLS> {
    type Output = SVector<T, ROWS>;

    fn mul(self, rhs: &'a SVector<T, COLS>) -> Self::Output {
        let mut output = SVector::zero();

        // TODO:

        output
    }
}
