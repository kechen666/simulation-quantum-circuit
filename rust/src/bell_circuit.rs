use crate::circuit::Instruction;
use crate::circuit::Circuit;
// use std::f64::consts::PI;

pub fn create_bell_circuit() -> Circuit {
    let mut circuit = Circuit::new(2);
    circuit.add_instruction(Instruction {
        name: "H".to_string(),
        qubits: vec![0],
        parameter: None,
    });
    circuit.add_instruction(Instruction {
        name: "CNOT".to_string(),
        qubits: vec![0, 1],
        parameter: None,
    });
    circuit
}
