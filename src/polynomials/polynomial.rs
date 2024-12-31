use std::{ops::Neg, usize};

use crate::{polynomials::polynomial_term::*, FFInt};

// TODO: Make the polynomial generic.

// TODO: Pretty printing using display.

// TODO: Make fields private.
// TODO: Write constructor(s).

/**
 * Multi-variate polynomial.
 */
#[derive(Debug)]
#[derive(Clone)]
pub struct Polynomial<const P: i64> {
    pub terms: Vec<PolynomialTerm<P>>
}

impl<const P: i64> Polynomial<P> {
    pub fn new() -> Polynomial::<P> {
        Polynomial::<P> {
            terms: vec![]
        }
    }

    // TODO: Check method. Assert that all terms have the same amount of variables (powers vector length).
    
    pub fn eval(&self, point: &Vec<FFInt<P>>) -> FFInt<P> {
        // TODO: assert that point length is the same as powers length

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
    pub fn partial_eval(&self, var_start: usize, point: &Vec<FFInt<P>>, len: usize) -> Self {
        let mut out = self.clone();

        // TODO: assert that point length is the same as powers length

        // sanity checks
        if (len == 0) || (var_start+len > point.len()) {
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
                        term.powers[idx] = 0; // Effectively remove the term as it has been evaluated.
                    }
                    // TODO: Negative powers?
                }
            }
        }

        out
    }
}

impl<const P: i64> std::ops::Index<usize> for Polynomial<P> {
    type Output = PolynomialTerm<P>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

impl<const P: i64> std::ops::IndexMut<usize> for Polynomial<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.terms[index]
    }
}

impl<const P: i64> std::ops::Add for Polynomial<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO: Powers size checks.

        // Start with lhs.
        let mut out = self.clone();

        // Continue with rhs
        'rhs_loop: for rhs_term in rhs.terms.iter() {
            
            for lhs_term in out.terms.iter_mut() {
                if vec_eq::<i64>(&lhs_term.powers, &rhs_term.powers) {
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

impl<const P: i64> std::ops::Neg for Polynomial<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = self.clone();

        for term in out.terms.iter_mut() {
            term.coefficient = -term.coefficient;
        }

        out
    }
}

impl<const P: i64> std::ops::Sub for Polynomial<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg() // self - rhs
    }
}

impl<const P: i64> std::ops::Mul<Self> for Polynomial<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // TODO: call rhs check
        // Assume that self is already checked and has the same amount of powers in all terms.
        // TODO: Better way?
        assert_eq!(self.terms[0].powers.len(), self.terms[0].powers.len());
        let var_count = self.terms[0].powers.len();

        let mut out = Self::new();

        for lhs_term in self.terms.iter() {
            for rhs_term in rhs.terms.iter() {
                let mut new_term = PolynomialTerm::<P> {
                    coefficient: (lhs_term.coefficient * rhs_term.coefficient),
                    powers: vec![0; var_count]
                };
                
                for idx in 0..var_count {
                    new_term.powers[idx] = lhs_term.powers[idx] + rhs_term.powers[idx];
                }

                out.terms.push(new_term);
            }
        }

        out
    }
}

impl<const P: i64> std::ops::Mul<FFInt<P>> for Polynomial<P> {
    type Output = Self;

    fn mul(self, rhs: FFInt<P>) -> Self::Output {
        let mut out = Self::new();

        for term in self.terms.iter() {
            let mut term = term.clone();
            term.coefficient = term.coefficient * rhs;
            out.terms.push(term);
        }

        out
    }
}