use crate::finite_fields::*;
use crate::polynomials::polynomial::*;

impl<const P: i64> Polynomial<P> {
    /**
     * Calculates the sum-check protocol' sum for a polynomial with VAR_COUNT variables and Integer Mod P coefficients.
     */
    pub fn sum_check(&self) -> FFInt<P> {
        // TODO: Check polynomial powers len.
        // TODO: Check we have at least one term.
        let var_count = self.terms[0].powers.len();

        let mut sum = FFInt::<P>::new(0);
        let mut combination: i64 = 0;

        // Sum-check's sum is the evaluation of the polynomial over all boolean inputs.
        while combination <= ((1 << var_count) - 1) {
            // TODO: Move out of loop for speed-up.
            let mut variables: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); var_count];
            let mut curr_combination = combination;

            for idx in 0..var_count {
                variables[idx] = FFInt::<P>::new(curr_combination & 1);
                curr_combination >>= 1;
            }

            sum = sum + self.eval(&variables);

            combination += 1;
        }

        sum
    }

    pub fn sum_check_iter(
        p: &Polynomial<P>,
        verifier_inputs: &Vec<FFInt<P>>,
        iter: usize // starts from 1 (as in round 1, similarly to the manuscript)
    ) -> Self {
        // TODO: Check input is the same size as polynomial variables.
        // TODO: Check polynomial.
        // TODO: Check polynomial has at least one term.
        let var_count = p.terms[0].powers.len();

        // Initial evaluation with known inputs from verifier.
        let p = p.partial_eval(0, verifier_inputs, iter-1);
        let mut out = Self::new();

        // TODO: Use slices.
        let mut variables: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); var_count];
        for idx in 0..(iter-1) {
            variables[idx] = verifier_inputs[idx];
        }
        
        let mut combination: i64 = 0;
        while combination <= ((1 << (var_count-iter)) - 1) {
            let mut curr_combination = combination;

            for idx in iter..var_count {
                variables[idx] = FFInt::<P>::new(curr_combination & 1);
                curr_combination >>= 1;
            }

            out = out + p.partial_eval(iter, &variables, var_count-iter);

            combination += 1;
        }

        out
    }
}