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
        p: Polynomial<P, VAR_COUNT>,
        verifier_inputs: &[FFInt<P>],
        iter: usize
    ) -> Self {
        let mut out = Polynomial::<P, VAR_COUNT>::new();
        
        let mut variables: [FFInt<P>; VAR_COUNT] = [FFInt::<P>::new(0); VAR_COUNT];
        for idx in 0..(iter-1) {
            variables[idx] = verifier_inputs[idx];            
        }

        // TODO: FINISH AND CHECK ME.
        let mut combination: i64 = 0;

        while combination <= ((1 << (VAR_COUNT-iter-1)) - 1) {
            let mut curr_combination = combination;

            for idx in (iter+1)..VAR_COUNT {
                variables[idx] = FFInt::<P>::new(curr_combination & 1);
                curr_combination >>= 1;
            }

            out = out + p.partial_eval(iter+1, &variables);

            combination += 1;
        }
        //

        out
    }
}

// TODO: Implement sum_check iteration function.
// Arguments:
//  - partially evaluated polynomial on which the protocol is run over.
//  - iteration of the protocol.
// Returns:
//  - The univariate polynomial after the sum check of some of the variables.