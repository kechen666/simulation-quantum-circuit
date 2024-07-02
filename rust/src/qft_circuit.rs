use crate::circuit::Instruction;
use crate::circuit::Circuit;
use std::f64::consts::PI;

pub fn create_qft_circuit(num_qubits: usize) -> Circuit {
    let mut circuit = Circuit::new(num_qubits);

    // Apply Hadamard gates 
    for i in 0..num_qubits {
        // Apply Hadamard gate to qubit i
        circuit.add_instruction(Instruction {
            name: "H".to_string(),
            qubits: vec![i],
            parameter: None,
        });
    }

    // Apply Hadamard gates and controlled rotations
    for i in 0..num_qubits {
        // Apply Hadamard gate to qubit i
        circuit.add_instruction(Instruction {
            name: "H".to_string(),
            qubits: vec![i],
            parameter: None,
        });

        // Apply controlled rotations
        for j in (i + 1)..num_qubits {
            circuit.add_instruction(Instruction {
                name: "CP".to_string(),
                qubits: vec![i, j],
                parameter: Some(2.0 * PI / (2usize.pow((j - i) as u32) as f64)),
            });
        }
    }

    // Apply SWAP gates 
    for i in 0..(num_qubits / 2) {
        // Apply Hadamard gate to qubit i
        circuit.add_instruction(Instruction {
            name: "SWAP".to_string(),
            qubits: vec![i,(num_qubits - i - 1)],
            parameter: None,
        });
    }
    
    circuit
}
