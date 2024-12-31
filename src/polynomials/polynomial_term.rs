use crate::finite_fields::*;

// TODO: Pretty printing using display.
// TODO: Make fields private.
// TODO: Write constructor.

/**
 * Describes the term of a polynomial.
 */
#[derive(Debug)]
#[derive(Clone)]
pub struct PolynomialTerm<const P: i64> {
    pub coefficient: FFInt<P>,
    pub powers: Vec<i64>
}

pub fn vec_eq<T: std::cmp::Eq>(arr1: &Vec<T>, arr2: &Vec<T>) -> bool {
    assert_eq!(arr1.len(), arr2.len());
    
    for idx in 0..arr1.len() {
        if arr1[idx] != arr2[idx] {
            return false;
        }
    }

    return true;
}