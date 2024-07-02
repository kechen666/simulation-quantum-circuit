use crate::circuit::Instruction;
use crate::circuit::Circuit;
// use std::f64::consts::PI;

pub fn create_ghz_circuit(num_qubits: usize) -> Circuit {
    let mut circuit = Circuit::new(num_qubits);

    // Apply Hadamard gate to the first qubit
    circuit.add_instruction(Instruction {
        name: "H".to_string(),
        qubits: vec![0],
        parameter: None,
    });

    // Apply CNOT gates between the first qubit and each subsequent qubit
    for i in 1..num_qubits {
        circuit.add_instruction(Instruction {
            name: "CNOT".to_string(),
            qubits: vec![0, i],
            parameter: None,
        });
    }

    circuit
}
