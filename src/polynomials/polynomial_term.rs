use crate::finite_fields::*;

// TODO: Pretty printing using display.

/**
 * Describes the term of a polynomial.
 */
#[derive(Debug)]
#[derive(Clone)]
pub struct PolynomialTerm<const P: i64> {
    pub coefficient: FFInt<P>,
    pub powers: Vec<i64>
}

pub fn arr_eq<T: std::cmp::Eq, const SIZE: usize>(arr1: &[T; SIZE], arr2: &[T; SIZE]) -> bool {
    for idx in 0..SIZE {
        if arr1[idx] != arr2[idx] {
            return false;
        }
    }

    return true;
}