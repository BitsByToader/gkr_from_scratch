use crate::polynomials::polynomial::*;
use crate::finite_fields::*;
use crate::circuits::*;
use crate::zero_knowledge::mle::*;

impl<const P: i64> Circuit<FFInt<P>> {
    /**
     * Returns the multilinear extension of the polynomial 'W' of layer 'layer_idx', as indicated in the manuscript.
     * Expects an already computed circuit.
     * The MLE is calculated using the evaluations of the polynomial, i.e. the values of the gates of the layer.
     */
    fn layer_mle_polynomial(&self, layer_idx: usize) -> Polynomial<P> {
        let mut gate_values: Vec<FFInt<P>> = vec![];
        
        // TODO: Reverse layers array so that output layer has index 0.
        for gate in self.layers[layer_idx].gates.iter() {
            gate_values.push(gate.value);
        }

        // TODO: change function to use all evaluations instead of 2^VAR_COUNT evaluations.
        mle_using_evaluations::<P>(&gate_values, 1)
    }

    /**
     * Returns the multilayer extension of the add polynomial of layer 'layer_idx'.
     * Expects an already computed circuit.
     */
    fn mult_mle_polynomial(&self, layer_idx: usize) -> Polynomial<P> {
        let mut evaluations: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); 3];
        
        // TODO: ???
        
        mle_using_evaluations::<P>(&evaluations, 3)
    }
}