use std::{ops::Neg, usize};

use crate::{polynomials::polynomial_term::*, FFInt};

// TODO: Pretty printing using display.

// TODO: Constructor using polynomial with higher VAR_COUNT
//      (also include where to place the old variables).

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
                        // Variable raised to power of 0 gets ignored in multiplication.
                        continue;
                    }
                    other => {
                        // Raise variable to its power via repeated multiplications.
                        for _ in 0..other {
                            term_value = term_value * (*var);
                        }
                    }
                    // TODO: Negative powers?
                }
            }

            res = res + term_value;
        }

        res
    }

    // TODO: Refactor with slices.
    pub fn partial_eval(&self, var_start: usize, point: &[FFInt<P>; VAR_COUNT], len: usize) -> Self {
        let mut out = self.clone();

        // sanity checks
        if (len == 0) || (var_start+len > VAR_COUNT) {
            // TODO: Return an Option here.
            return out;
        }

        for term in out.terms.iter_mut() {
            for idx in var_start..(var_start+len) {
                match term.powers[idx] {
                    0 => {
                        // Variable raised to power of 0 gets ignored in multiplication.
                        continue;
                    }
                    other => {
                        // Raise variable to its power via repeated multiplications.
                        for _ in 0..other {
                            term.coefficient = term.coefficient * point[idx];
                        }
                        term.powers[idx] = 0; // Cancel out the evaluated variable.
                    }
                    // TODO: Negative powers?
                }
            }
        }

        out
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
        // Start with lhs.
        let mut out = self.clone();

        // 
        'rhs_loop: for rhs_term in rhs.terms.iter() {
            
            for lhs_term in out.terms.iter_mut() {
                if arr_eq::<i64, VAR_COUNT>(&lhs_term.powers, &rhs_term.powers) {
                    lhs_term.coefficient = lhs_term.coefficient + rhs_term.coefficient;
                    
                    // If a match in powers was found, skip to next term.
                    continue 'rhs_loop;      
                }
            }
            
            // Match in powers not found, add term to lhs.
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

impl<const P: i64, const VAR_COUNT: usize> std::ops::Mul for Polynomial<P, VAR_COUNT> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::new();

        for lhs_term in self.terms.iter() {
            for rhs_term in rhs.terms.iter() {
                let mut new_term = PolynomialTerm::<P, VAR_COUNT> {
                    coefficient: (lhs_term.coefficient * rhs_term.coefficient),
                    powers: [0; VAR_COUNT]
                };
                
                for idx in 0..VAR_COUNT {
                    new_term.powers[idx] = lhs_term.powers[idx] + rhs_term.powers[idx];
                }

                out.terms.push(new_term);
            }
        }

        out
    }
}