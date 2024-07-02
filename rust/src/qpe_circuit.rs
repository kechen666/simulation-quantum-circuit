use crate::circuit::Instruction;
use crate::circuit::Circuit;
use std::f64::consts::PI;

pub fn create_qpe_circuit(num_qubits: usize) -> Circuit {
    let mut circuit = Circuit::new(num_qubits+1);

    // Apply Hadamard gate to the target qubit
    circuit.add_instruction(Instruction::new("H", vec![num_qubits], None));


    // Apply Hadamard gates to counting qubits
    for qubit in 0..num_qubits {
        circuit.add_instruction(Instruction::new("H", vec![qubit], None));
    }

    // Apply controlled-U operations
    for control in 0..num_qubits {
        let repetitions = 2usize.pow(control as u32);
        for _ in 0..repetitions {
            let theta = 2.0 * PI * 0.25; // Assuming θ = 0.25
            circuit.add_instruction(Instruction::new("CP", vec![control, num_qubits], Some(theta)));
        }
    }

    // Apply SWAP gates 
    for qubit in 0..(num_qubits/2) {
        circuit.add_instruction(Instruction::new("SWAP", vec![qubit, num_qubits - qubit - 1], None));
    }


    // Inverse QFT
    for qubit in 0..num_qubits {
        for k in 0..qubit {
            let theta = -PI / (2usize.pow((qubit - k) as u32) as f64);
            circuit.add_instruction(Instruction::new("CP", vec![k, qubit], Some(theta)));
        }
        circuit.add_instruction(Instruction::new("H", vec![qubit], None));
    }
    
    circuit
}
