use crate::finite_fields::*;
use crate::polynomials::polynomial::*;

impl<const P: i64, const VAR_COUNT: usize> Polynomial<P, VAR_COUNT> {
    /**
     * Calculates the sum-check protocol' sum for a polynomial with VAR_COUNT variables and Integer Mod P coefficients.
     */
    pub fn sum_check(&self) -> FFInt<P> {
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

            sum = sum + self.eval(&variables);

            combination += 1;
        }

        sum
    }

    pub fn sum_check_iter(
        p: &Polynomial<P, VAR_COUNT>,
        verifier_inputs: &[FFInt<P>; VAR_COUNT],
        iter: usize // starts from 1 (as in round 1, similarly to the manuscript)
    ) -> Self {
        // Initial evaluation with known inputs from verifier.
        let p = p.partial_eval(0, verifier_inputs, iter-1);
        let mut out = Self::new();

        let mut variables: [FFInt<P>; VAR_COUNT] = [FFInt::<P>::new(0); VAR_COUNT];
        // TODO: Use slices.
        for idx in 0..(iter-1) {
            variables[idx] = verifier_inputs[idx];
        }
        
        // TODO: FINISH AND CHECK ME.
        let mut combination: i64 = 0;

        while combination <= ((1 << (VAR_COUNT-iter)) - 1) {
            let mut curr_combination = combination;

            for idx in iter..VAR_COUNT {
                variables[idx] = FFInt::<P>::new(curr_combination & 1);
                curr_combination >>= 1;
            }

            out = out + p.partial_eval(iter, &variables, VAR_COUNT-iter);

            combination += 1;
        }
        //

        out
    }
}