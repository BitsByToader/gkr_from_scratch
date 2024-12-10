use std::ops::*;

use crate::finite_fields::*;

// TODO: Should this Polynomial struct implement its coefficients as an array or as a vector?

pub struct Polynomial<const P: i64, const COEFF_LEN: usize> {
    pub coefficients: [FFInt<P>; COEFF_LEN]
}

impl<const P: i64, const COEFF_LEN: usize> Polynomial<P, COEFF_LEN> {
    pub fn new() -> Polynomial::<P, COEFF_LEN> {
        Polynomial::<P, COEFF_LEN> {
            coefficients:[FFInt::<P>::new(0); COEFF_LEN]
        }
    }
}

impl<const P: i64, const COEFF_LEN: usize> std::ops::Index<usize> for Polynomial<P, COEFF_LEN> {
    type Output = FFInt<P>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<const P: i64, const COEFF_LEN: usize> std::ops::IndexMut<usize> for Polynomial<P, COEFF_LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<const P: i64, const COEFF_LEN: usize> std::ops::Add for Polynomial<P, COEFF_LEN> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self::new();

        for idx in 0..self.coefficients.len() {
            out[idx] = self[idx] + rhs[idx];
        }

        out
    }
}

impl<const P: i64, const COEFF_LEN: usize> std::ops::Neg for Polynomial<P, COEFF_LEN> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = Self::new();

        for idx in 0..self.coefficients.len() {
            out[idx] = -self.coefficients[idx];
        }

        out
    }
}

impl<const P: i64, const COEFF_LEN: usize> std::ops::Sub for Polynomial<P, COEFF_LEN> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = Self::new();

        for idx in 0..self.coefficients.len() {
            out[idx] = self[idx] - rhs[idx];
        }

        out
    }
}