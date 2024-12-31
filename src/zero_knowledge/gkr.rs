use crate::polynomials::polynomial::*;
use crate::finite_fields::*;
use crate::circuits::*;
use crate::zero_knowledge::mle::*;

/**
 * Extracts the bits of an int to a vector of ints of size 'limit'.
 * Used in the GKR protocol when evaluating a MLE using a gate label, as the MLE will have 'limit' variables.
 */
fn int_to_bit_vector<const P: i64>(value: i64, limit: usize) -> Vec<i64> {
    let mut out: Vec<i64> = vec![];
    let mut value = value;

    for _ in 0..limit {
        out.push(value & 1);
        value = value >> 1;
    }

    out
}

impl<const P: i64> Circuit<FFInt<P>> {
    /**
     * Returns the multilinear extension of the polynomial 'W' of layer 'layer_idx', as indicated in the manuscript.
     * Expects an already computed circuit.
     * The MLE is calculated using the evaluations of the polynomial, i.e. the values of the gates of the layer.
     * The resulting polynomial will have log_2(gate count of layer) variables!
     */
    fn layer_mle_polynomial(&self, layer_idx: usize) -> Polynomial<P> {
        let mut gate_values: Vec<FFInt<P>> = vec![];
        
        // Output layer is labeled layer 0 in the GRK protocol.
        let gates = &self.layers[self.layers.len()-layer_idx-1].gates;
        // GKR protocol constrains layers to have a power of 2 gates.
        let var_count = gates.len().next_power_of_two();

        for gate in gates.iter() {
            gate_values.push(gate.value);
        }

        mle_using_evaluations::<P>(&gate_values, var_count)
    }

    /**
     * Returns the multilayer extension of the mult polynomial of layer 'layer_idx'.
     * Expects an already computed circuit.
     */
    fn mult_mle_polynomial(&self, layer_idx: usize) -> Polynomial<P> {
        let curr_layer_gates = &self.layers[self.layers.len()-layer_idx-1].gates;
        let next_layer_gates = &self.layers[self.layers.len()-layer_idx-2].gates;
        
        let curr_layer_gate_count = curr_layer_gates.len();
        let next_layer_gate_count = next_layer_gates.len();

        // No mult gates initially, fill later when iterating over layer.
        let mut evaluations: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); curr_layer_gate_count * 2 * next_layer_gate_count];
        
        for (gate_idx, gate) in curr_layer_gates.iter().enumerate() {
            match gate.gate_type {
                GateType::Multiplier((in1, in2)) => {
                    evaluations[gate_idx*2*next_layer_gate_count + in1*next_layer_gate_count + in2] = FFInt::<P>::new(1);
                },
                _ => () // Ignore Input gates and Adder gates, as they are already zero-ed out.
            }
        }
        
        let curr_layer_var_count = curr_layer_gate_count.next_power_of_two();
        let next_layer_var_count = next_layer_gate_count.next_power_of_two();
        mle_using_evaluations::<P>(&evaluations, curr_layer_var_count + 2 * next_layer_var_count)
    }

    /**
     * Returns the multilayer extension of the add polynomial of layer 'layer_idx'.
     * Expects an already computed circuit.
     */
    fn add_mle_polynomial(&self, layer_idx: usize) -> Polynomial<P> {
        let curr_layer_gates = &self.layers[self.layers.len()-layer_idx-1].gates;
        let next_layer_gates = &self.layers[self.layers.len()-layer_idx-2].gates;
        
        let curr_layer_gate_count = curr_layer_gates.len();
        let next_layer_gate_count = next_layer_gates.len();

        // No add gates initially, fill later when iterating over layer.
        let mut evaluations: Vec<FFInt<P>> = vec![FFInt::<P>::new(0); curr_layer_gate_count * 2 * next_layer_gate_count];
        
        for (gate_idx, gate) in curr_layer_gates.iter().enumerate() {
            match gate.gate_type {
                GateType::Adder((in1, in2)) => {
                    evaluations[gate_idx*2*next_layer_gate_count + in1*next_layer_gate_count + in2] = FFInt::<P>::new(1);
                },
                _ => () // Ignore Input gates and Adder gates, as they are already zero-ed out.
            }
        }
        
        let curr_layer_var_count = curr_layer_gate_count.next_power_of_two();
        let next_layer_var_count = next_layer_gate_count.next_power_of_two();
        mle_using_evaluations::<P>(&evaluations, curr_layer_var_count + 2 * next_layer_var_count)
    }
}
