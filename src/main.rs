use std::ops::Neg;

use crate::circuits::*;
use crate::finite_fields::*;

mod circuits;
mod finite_fields;

// TODO: Implement proper tests...

fn main() {
    println!("~~GKR from scratch~~");
    
    let mut circuit = Circuit {
        layers: vec![
        // Layer 0 - Input Layer
        Layer {
            gates: vec![
            Gate {
                gate_type: GateType::Input,
                value: 3,
            },
            Gate {
                gate_type: GateType::Input,
                value: 2,
            },
            Gate {
                gate_type: GateType::Input,
                value: 3,
            },
            Gate {
                gate_type: GateType::Input,
                value: 1,
            }
            ],
        },
        
        // Layer 1
        Layer {
            gates: vec![
                Gate {
                    gate_type: GateType::Multiplier((0, 0)),
                    value: 0,
                },
                Gate {
                    gate_type: GateType::Multiplier((1, 1)),
                    value: 0,
                },
                Gate {
                    gate_type: GateType::Multiplier((1, 2)),
                    value: 0,
                },
                Gate {
                    gate_type: GateType::Multiplier((3, 3)),
                    value: 0,
                }
            ]
        },
        
        // Layer 2 - Output Layer
        Layer {
            gates: vec![
                Gate {
                    gate_type: GateType::Multiplier((0, 1)),
                    value: 0,
                },
                Gate {
                    gate_type: GateType::Multiplier((2, 3)),
                    value: 0,
                }
            ]
        }
        ],
    };
    
    // Print blank (input only) circuit.
    println!("{:?}", circuit);
    
    // Compute circuit gates' values.
    circuit.compute();
    
    // Print output (last) layer 
    println!("{:?}", circuit.layers.last().unwrap());

    // FF basic tests...
    let i1 = FFInt::<5>::new(-7);
    println!("{:?}", i1);
    println!("{:?}", -i1);
}