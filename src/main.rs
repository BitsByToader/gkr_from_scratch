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

    println!();
    println!();

    // FF basic tests...
    let i1 = FFInt::<5>::new(-7);
    println!("i1  = {:?}", i1);
    println!("-i1 = {:?}", -i1);
    
    println!();

    // FF inverse tests...
    let i2 = FFInt::<23>::new(7);
    println!("i2        = {:?}", i2);
    println!("i2inverse = {:?}", i2.inverse());
    println!("inverse field axiom: {:?}", i2 * i2.inverse());
    
    println!();
    println!();

    // Polynomials tests...
    let p = Polynomial::<211> {
        terms: vec![
            PolynomialTerm::<211> {
                coefficient: FFInt::<211>::new(2),
                powers: vec![3, 0, 0]
            },
            PolynomialTerm::<211> {
                coefficient: FFInt::<211>::new(1),
                powers: vec![1, 0, 1]
            },
            PolynomialTerm::<211> {
                coefficient: FFInt::<211>::new(1),
                powers: vec![0, 1, 1]
            }
        ]
    };
    println!("Polynomial used: {:#?}", p);
    
    // SUM-CHECK PROTOCOL EXAMPLE EXECUTION
    // TODO: This 'test' should also check (print) the degree of each variable of interest.
    println!("Sum-check sum for polynomial: {:?}", p.sum_check());
    let mut verifier = vec![FFInt::<211>::new(0);3];

    let iter1 = Polynomial::<211>::sum_check_iter(&p, &verifier, 1);
    println!("Iteration1 polynomial: {:#?}", iter1);

    verifier[0] = FFInt::<211>::new(2);
    let iter2 = Polynomial::<211>::sum_check_iter(&p, &verifier, 2);
    println!("Iteration2 polynomial: {:#?}", iter2);

    verifier[1] = FFInt::<211>::new(3);
    let iter3 = Polynomial::<211>::sum_check_iter(&p, &verifier, 3);
    println!("Iteration3 polynomial: {:#?}", iter3);

    let check1 = iter3.eval(&vec![FFInt::<211>::new(0),FFInt::<211>::new(0), FFInt::<211>::new(6)]);
    let check2 = p.eval(&vec![FFInt::<211>::new(2),FFInt::<211>::new(3), FFInt::<211>::new(6)]);
    println!("Final check: {:?} == {:?}", check1, check2);

    println!();
    println!();

    // Multi Linear Extensions
    let evaluations = vec![
        FFInt::<5>::new(1),
        FFInt::<5>::new(2),
        FFInt::<5>::new(1),
        FFInt::<5>::new(4),
    ];
    let extension = zero_knowledge::mle::mle_using_evaluations::<5>(&evaluations, 2);

    for i in 0..5 {
        for j in 0..5 {
            println!("Evaluation({i},{j})={:?}", extension.eval(&vec![FFInt::<5>::new(i), FFInt::<5>::new(j)]))
        }
    }
}
