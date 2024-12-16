use crate::circuits::*;
use crate::finite_fields::*;
use crate::polynomials::polynomial::*;
use crate::polynomials::polynomial_term::*;
use crate::zero_knowledge::sum_check::*;

mod circuits;
mod finite_fields;
mod polynomials;
mod zero_knowledge;

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
    println!("{:#?}", circuit);
    
    // Compute circuit gates' values.
    circuit.compute();
    
    // Print output (last) layer 
    println!("{:#?}", circuit.layers.last().unwrap());

    // FF basic tests...
    let i1 = FFInt::<5>::new(-7);
    println!("i1  = {:?}", i1);
    println!("-i1 = {:?}", -i1);
    
    // FF inverse tests...
    let i2 = FFInt::<23>::new(7);
    println!("i2        = {:?}", i2);
    println!("i2inverse = {:?}", i2.inverse());
    println!("inverse field axiom: {:?}", i2 * i2.inverse());
    
    // Polynomials tests...
    let p = Polynomial::<29, 3> {
        terms: vec![
            PolynomialTerm::<29, 3> {
                coefficient: FFInt::<29>::new(2),
                powers: [3, 0, 0]
            },
            PolynomialTerm::<29, 3> {
                coefficient: FFInt::<29>::new(1),
                powers: [1, 0, 1]
            },
            PolynomialTerm::<29, 3> {
                coefficient: FFInt::<29>::new(1),
                powers: [0, 1, 1]
            }
        ]
    };
    println!("Polynomial used: {:#?}", p);
    println!("Sum-check sum for polynomial: {:?}", p.sum_check());
}