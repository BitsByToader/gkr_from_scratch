use std::ops::IndexMut;

/**
 * This enum variant defines what forms a gate in a circuit can take.
 * The 'input layer' of a gate represents the layer which the gate uses as its inputs.
 * This layer can be the previous layer, if the circuit is seen from input to output.
 * Similarly, this layer can be the next layer, if the circuit is seen from output to input.
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum GateType {
    /**
     * This gate holds known, input data, does no computations.
     */
    Input,

    /**
     * This gate computes the sum of its inputs.
     * The tuple represents indexes in the array of gates of its input layer.
     */
    Adder((usize, usize)),

    /**
     * This gate computes the products of its inputs.
     * The tuple represents indexes in the array of gates of its input layer.
     */
    Multiplier((usize, usize)),
}

/**
 * A gate holds a value in the circuit, and depdending on its type, what kind of computation it performs.
 */
#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Gate<T: std::ops::Add<Output=T> + std::ops::Mul<Output=T> + Clone + Copy> {
    pub gate_type: GateType,
    pub value: T,
}

/**
 * A layers holds multiple gates in the circuit which sit on the same level.
 */
#[derive(Debug)]
pub struct Layer<T: std::ops::Add<Output=T> + std::ops::Mul<Output=T> + Clone + Copy> {
    pub gates: Vec<Gate<T>>,
}

/**
 * A circuit is formed of multiple layers of fan-in 2 gates.
 * All gates on the last layer are seen as output gates.
 * Gates on any layer can be inputs, if that is their type.
 */
#[derive(Debug)]
pub struct Circuit<T: std::ops::Add<Output=T> + std::ops::Mul<Output=T> + Clone + Copy> {
    pub layers: Vec<Layer<T>>,
}

impl<T: std::ops::Add<Output=T> + std::ops::Mul<Output=T> + Clone + Copy> Circuit<T> {
    // TODO: Circuit validation method.
    // (mostly to check the indexes of the compute gates)

    /**
     * Returns the copy of the gate, but with its value computed using the input (i.e. previous) layer. 
     */
    fn compute_gate(gate: &Gate<T>, input_layer: &Layer<T>) -> Gate<T> {
        let mut new_gate = gate.clone();

        new_gate.value = match new_gate.gate_type {
            // Input gate mantains its value since gate doesn't compute anything.
            GateType::Input => gate.value,
            
            GateType::Adder(inputs) => {
                input_layer.gates[inputs.0].value + input_layer.gates[inputs.1].value
            },
            
            GateType::Multiplier(inputs) => {
                input_layer.gates[inputs.0].value * input_layer.gates[inputs.1].value
            },
        };

        new_gate
    }

    /**
     * Computes the values of the gates in the circuit.
     */
    pub fn compute(&mut self) {
        // Layer 0 is always input Layer, no computations needed, so skip it.
        for layer_idx in 1..self.layers.len() {
            for gate_idx in 0..self.layers[layer_idx].gates.len() {
                let computed_gate = Circuit::compute_gate(&self.layers[layer_idx].gates[gate_idx], &self.layers[layer_idx-1]);
                *(self.layers[layer_idx].gates.index_mut(gate_idx)) = computed_gate;
            }
        }
    }
}