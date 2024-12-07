#![allow(dead_code)]

#[derive(Debug)]
enum GateType {
    Input,
    Adder((usize, usize)),
    Multiplier((usize, usize)), // The tuple represents indexes in the array of gates in the next layer (seen as out->in)
}

#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    value: u32,
}

#[derive(Debug)]
struct Layer {
    gates: Vec<Gate>,
}

#[derive(Debug)]
struct Circuit {
    layers: Vec<Layer>,
}

fn main() {
    println!("~~GKR from scratch~~");
    
    let circuit = Circuit {
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
    
    println!("{:?}", circuit)
}
