use crate::{finite_fields::*, PolynomialTerm};
use crate::polynomials::polynomial::*;

pub fn basis_polynomial<const P: i64, const VAR_COUNT: usize>(point: &[FFInt<P>; VAR_COUNT]) -> Polynomial<P, VAR_COUNT> {
    let mut out = Polynomial::<P, VAR_COUNT> {
        terms: vec![
            PolynomialTerm::<P, VAR_COUNT> {
                coefficient: FFInt::<P>::new(1),
                powers: [0; VAR_COUNT]
            }
        ]
    };

    for idx in 0..VAR_COUNT {
        let mut temp = Polynomial::<P, VAR_COUNT> {
            terms: vec![
                PolynomialTerm::<P, VAR_COUNT> {
                    coefficient: FFInt::<P>::new(1) - point[idx],
                    powers: [0; VAR_COUNT]
                },
                PolynomialTerm::<P, VAR_COUNT> {
                    coefficient: FFInt::<P>::new(2) * point[idx] - FFInt::<P>::new(1),
                    powers: [0; VAR_COUNT]
                }
            ]
        };
        temp.terms[1].powers[idx] = 1;

        out = out * temp;
    }

    out
}

pub fn mle_using_evaluations<const P: i64, const VAR_COUNT: usize>(evaluations: &Vec<FFInt<P>>) -> Polynomial<P, VAR_COUNT> {
    let mut out = Polynomial::<P, VAR_COUNT>::new();
    let mut combination: i64 = 0;

    while combination <= ((1 << VAR_COUNT) - 1) {
        let mut variables: [FFInt<P>; VAR_COUNT] = [FFInt::<P>::new(0); VAR_COUNT];
        let mut curr_combination = combination;

        for idx in 0..VAR_COUNT {
            variables[idx] = FFInt::<P>::new(curr_combination & 1);
            curr_combination >>= 1;
        }

        // Will panic if provided vector is not of size 2^VAR_COUNT
        out = out + (basis_polynomial(&variables) * evaluations[usize::try_from(combination).unwrap()]);

        combination += 1;
    }

    out
}

impl<const P: i64, const VAR_COUNT: usize> Polynomial<P, VAR_COUNT> {
    pub fn mle(&self) -> Self {
        let mut out = Polynomial::<P, VAR_COUNT>::new();
        let mut combination: i64 = 0;

        while combination <= ((1 << VAR_COUNT) - 1) {
            let mut variables: [FFInt<P>; VAR_COUNT] = [FFInt::<P>::new(0); VAR_COUNT];
            let mut curr_combination = combination;

            for idx in 0..VAR_COUNT {
                variables[idx] = FFInt::<P>::new(curr_combination & 1);
                curr_combination >>= 1;
            }

            out = out + (basis_polynomial(&variables) * self.eval(&variables));

            combination += 1;
        }

        out
    }
}