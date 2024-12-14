use crate::finite_fields::*;
use crate::polynomials::polynomial::*;

/**
 * Calculates the sum-check protocol' sum for a polynomial with VAR_COUNT variables and Integer Mod P coefficients.
 */
pub fn sum_check<const VAR_COUNT: usize, const P: i64>(polynomial: &Polynomial<P, VAR_COUNT>) -> FFInt<P> {
    let mut sum = FFInt::<P>::new(0);
    let mut combination: i64 = 0;

    // Sum-check's sum is the evaluation of the polynomial over all boolean inputs.
    while combination <= ((1 << VAR_COUNT) - 1) {
        let mut variables: [FFInt<P>; VAR_COUNT] = [FFInt::<P>::new(0); VAR_COUNT];
        let mut curr_combination = combination;

        for idx in 0..VAR_COUNT {
            variables[idx] = FFInt::<P>::new(curr_combination & 1);
            curr_combination >>= 1;
        }

        sum = sum + polynomial.eval(&variables);

        combination += 1;
    }

    sum
}