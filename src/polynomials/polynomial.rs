use std::ops::Neg;

use crate::{polynomials::polynomial_term::*, FFInt};

/**
 * Multi-variate polynomial.
 */
#[derive(Debug)]
#[derive(Clone)]
 pub struct Polynomial<const P: i64, const VAR_COUNT: usize> {
    pub terms: Vec<PolynomialTerm<P, VAR_COUNT>>
}

impl<const P: i64, const VAR_COUNT: usize> Polynomial<P, VAR_COUNT> {
    pub fn new() -> Polynomial::<P, VAR_COUNT> {
        Polynomial::<P, VAR_COUNT> {
            terms: vec![]
        }
    }

    pub fn eval(&self, point: &[FFInt<P>; VAR_COUNT]) -> FFInt<P> {
        let mut res = FFInt::<P>::new(0);

        for term in &self.terms {
            let mut term_value = term.coefficient;

            for (idx, var) in point.iter().enumerate() {
                match term.powers[idx] {
                    0 => {
                        // Variable raised to power of 0 gets canceled in multiplication.
                        continue;
                    }
                    other => {
                        // Raise variable to its power via repeated multiplications.
                        for _ in 0..other {
                            term_value = term_value * (*var);
                        }
                    }
                }
            }

            res = res + term_value;
        }

        res
    }
}

impl<const P: i64, const VAR_COUNT: usize> std::ops::Index<usize> for Polynomial<P, VAR_COUNT> {
    type Output = PolynomialTerm<P, VAR_COUNT>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

impl<const P: i64, const VAR_COUNT: usize> std::ops::IndexMut<usize> for Polynomial<P, VAR_COUNT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.terms[index]
    }
}

impl<const P: i64, const VAR_COUNT: usize> std::ops::Add for Polynomial<P, VAR_COUNT> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();

        'rhs_loop: for rhs_term in rhs.terms.iter() {
            
            for lhs_term in out.terms.iter_mut() {
                if arr_eq::<i64, VAR_COUNT>(&lhs_term.powers, &rhs_term.powers) {
                    lhs_term.coefficient = lhs_term.coefficient + rhs_term.coefficient;
                    
                    // If a match in powers was found, skip to next term to add.
                    continue 'rhs_loop;      
                }
            }
            
            out.terms.push(rhs_term.clone());
        }

        out
    }
}

impl<const P: i64, const VAR_COUNT: usize> std::ops::Neg for Polynomial<P, VAR_COUNT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = self.clone();

        for term in out.terms.iter_mut() {
            term.coefficient = -term.coefficient;
        }

        out
    }
}

impl<const P: i64, const VAR_COUNT: usize> std::ops::Sub for Polynomial<P, VAR_COUNT> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg() // self - rhs
    }
}