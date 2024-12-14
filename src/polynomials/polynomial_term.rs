use crate::finite_fields::*;

/**
 * Describes the term of a polynomial.
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct PolynomialTerm<const P: i64, const VAR_COUNT: usize> {
    pub coefficient: FFInt<P>,
    pub powers: [i64; VAR_COUNT]
}

pub fn arr_eq<T: std::cmp::Eq, const SIZE: usize>(arr1: &[T; SIZE], arr2: &[T; SIZE]) -> bool {
    for idx in 0..SIZE {
        if arr1[idx] != arr2[idx] {
            return false;
        }
    }

    return true;
}