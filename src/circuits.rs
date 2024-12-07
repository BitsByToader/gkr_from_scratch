/**
 * This enum variant defines what forms a gate in a circuit can take.
 * The 'input layer' of a gate represents the layer which the gate uses as its inputs.
 * This layer can be the previous layer, if the circuit is seen from input to output.
 * Similarly, this layer can be the next layer, if the circuit is seen from output to input.
 */
#[derive(Debug)]
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
pub struct Gate {
    pub gate_type: GateType,
    pub value: u32,
}

/**
 * A layers holds multiple gates in the circuit which sit on the same level.
 */
#[derive(Debug)]
pub struct Layer {
    pub gates: Vec<Gate>,
}

/**
 * A circuit is formed of multiple layers of fan-in 2 gates.
 * All gates on the last layer are seen as output gates.
 * Gates on any layer can be inputs, if that is their type.
 */
#[derive(Debug)]
pub struct Circuit {
    pub layers: Vec<Layer>,
}