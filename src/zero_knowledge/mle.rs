use crate::{finite_fields::*, PolynomialTerm};
use crate::polynomials::polynomial::*;

pub fn basis_polynomial<const P: i64>(point: &Vec<FFInt<P>>, v: usize) -> Polynomial<P> {
    let mut out = Polynomial::<P> {
        terms: vec![
            PolynomialTerm::<P> {
                coefficient: FFInt::<P>::new(1),
                powers: vec![0; v]
            }
        ]
    };

    for idx in 0..v {
        let mut temp = Polynomial::<P> {
            terms: vec![
                PolynomialTerm::<P> {
                    coefficient: FFInt::<P>::new(1) - point[idx],
                    powers: vec![0; v]
                },
                PolynomialTerm::<P> {
                    coefficient: FFInt::<P>::new(2) * point[idx] - FFInt::<P>::new(1),
                    powers: vec![0; v]
                }
            ]
        };
        temp.terms[1].powers[idx] = 1;

        out = out * temp;
    }

    out
}

/**
 * Calculates the multi linear extension of a polynomial described by its evaluations over the v-variate domain.
 */
pub fn mle_using_evaluations<const P: i64>(evaluations: &Vec<FFInt<P>>, v: usize) -> Polynomial<P> {
    let mut out = Polynomial::<P>::new();
    let mut combination: i64 = 0;

    while combination <= ((1 << v) - 1) {
        let mut variables: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); v];
        let mut curr_combination = combination;

        for idx in 0..v {
            variables[idx] = FFInt::<P>::new(curr_combination & 1);
            curr_combination >>= 1;
        }

        // Will panic if provided vector is not of size 2^v
        out = out + (basis_polynomial(&variables, v) * evaluations[usize::try_from(combination).unwrap()]);

        combination += 1;
    }

    out
}

impl<const P: i64> Polynomial<P> {
    pub fn mle(&self) -> Self {
        // TODO: Check input is the same size as polynomial variables.
        // TODO: Check polynomial.
        // TODO: Check polynomial has at least one term.
        let var_count = self.terms[0].powers.len();

        let mut out = Polynomial::<P>::new();
        let mut combination: i64 = 0;

        while combination <= ((1 << var_count) - 1) {
            let mut variables: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); var_count];
            let mut curr_combination = combination;

            for idx in 0..var_count {
                variables[idx] = FFInt::<P>::new(curr_combination & 1);
                curr_combination >>= 1;
            }

            out = out + (basis_polynomial(&variables, var_count) * self.eval(&variables));

            combination += 1;
        }

        out
    }
}