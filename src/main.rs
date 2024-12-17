use crate::circuits::*;
use crate::finite_fields::*;
use crate::polynomials::polynomial::*;
use crate::polynomials::polynomial_term::*;

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
    let p = Polynomial::<211, 3> {
        terms: vec![
            PolynomialTerm::<211, 3> {
                coefficient: FFInt::<211>::new(2),
                powers: [3, 0, 0]
            },
            PolynomialTerm::<211, 3> {
                coefficient: FFInt::<211>::new(1),
                powers: [1, 0, 1]
            },
            PolynomialTerm::<211, 3> {
                coefficient: FFInt::<211>::new(1),
                powers: [0, 1, 1]
            }
        ]
    };
    println!("Polynomial used: {:#?}", p);
    
    // SUM-CHECK PROTOCOL EXAMPLE EXECUTION
    println!("Sum-check sum for polynomial: {:?}", p.sum_check());
    let mut verifier = [FFInt::<211>::new(0);3];

    let iter1 = Polynomial::<211, 3>::sum_check_iter(&p, &verifier, 1);
    println!("Iteration1 polynomial: {:#?}", iter1);

    verifier[0] = FFInt::<211>::new(2);
    let iter2 = Polynomial::<211, 3>::sum_check_iter(&p, &verifier, 2);
    println!("Iteration2 polynomial: {:#?}", iter2);

    verifier[1] = FFInt::<211>::new(3);
    let iter3 = Polynomial::<211, 3>::sum_check_iter(&p, &verifier, 3);
    println!("Iteration3 polynomial: {:#?}", iter3);

    let check1 = iter3.eval(&[FFInt::<211>::new(0),FFInt::<211>::new(0), FFInt::<211>::new(6)]);
    let check2 = p.eval(&[FFInt::<211>::new(2),FFInt::<211>::new(3), FFInt::<211>::new(6)]);
    println!("Final check: {:?} == {:?}", check1, check2);
}