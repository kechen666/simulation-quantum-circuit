use ndarray::{Array2, Array1, ArrayView2, array};
use num_complex::Complex;
// use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: String,
    pub qubits: Vec<usize>,
    pub parameter: Option<f64>,
}

impl Instruction {
    pub fn new(name: &str, qubits: Vec<usize>, parameter: Option<f64>) -> Self {
        Self {
            name: name.to_string(),
            qubits,
            parameter,
        }
    }
}
// pub struct Instruction {
//     pub name: String,
//     pub qubits: Vec<usize>,
//     pub parameter: Option<f64>,
// }



#[derive(Debug)]
pub struct Circuit {
    pub num_qubits: usize,
    pub instructions: Vec<Instruction>,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Self {
        Circuit {
            num_qubits,
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn simulate(&self) -> Array1<Complex<f64>> {
        let mut state = Array1::<Complex<f64>>::zeros(1 << self.num_qubits);
        state[0] = Complex::new(1.0, 0.0);

        for instr in &self.instructions {
            match instr.name.as_str() {
                "H" => state = self.apply_single_qubit_gate(&state, self.hadamard_gate(), instr.qubits[0]),
                "X" => state = self.apply_single_qubit_gate(&state, self.x_gate(), instr.qubits[0]),
                "CNOT" => state = self.apply_cnot(&state, instr.qubits[0], instr.qubits[1]),
                "CP" => state = self.apply_cp(&state, instr.qubits[0], instr.qubits[1], instr.parameter.unwrap()),
                "SWAP" => state = self.apply_swap(&state, instr.qubits[0], instr.qubits[1]),
                _ => (),
            }
        }

        state
    }

    fn hadamard_gate(&self) -> Array2<Complex<f64>> {
        let h = Complex::new(1.0 / (2.0 as f64).sqrt(), 0.0);
        array![
            [h, h],
            [h, -h]
        ]
    }

    fn x_gate(&self) -> Array2<Complex<f64>> {
        array![
            [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]
        ]
    }

    fn apply_single_qubit_gate(&self, state: &Array1<Complex<f64>>, gate: Array2<Complex<f64>>, qubit: usize) -> Array1<Complex<f64>> {
        let mut full_gate = Array2::<Complex<f64>>::eye(1);
        for i in 0..self.num_qubits {
            if i == qubit {
                full_gate = kron(full_gate.view(), gate.view());
            } else {
                full_gate = kron(full_gate.view(), Array2::eye(2).view());
            }
        }
        full_gate.dot(state)
    }

    fn apply_cnot(&self, state: &Array1<Complex<f64>>, control: usize, target: usize) -> Array1<Complex<f64>> {
        let dim = 1 << self.num_qubits;
        let mut cnot = Array2::<Complex<f64>>::eye(dim);

        for i in 0..dim {
            let binary = format!("{:0width$b}", i, width = self.num_qubits);
            if binary.chars().nth(control).unwrap() == '1' {
                let mut flipped = binary.bytes().collect::<Vec<u8>>();
                flipped[target] = if binary.chars().nth(target).unwrap() == '1' { b'0' } else { b'1' };
                let j = usize::from_str_radix(std::str::from_utf8(&flipped).unwrap(), 2).unwrap();
                cnot[[i, i]] = Complex::new(0.0, 0.0);
                cnot[[i, j]] = Complex::new(1.0, 0.0);
            }
        }

        cnot.dot(state)
    }

    fn apply_cp(&self, state: &Array1<Complex<f64>>, control: usize, target: usize, theta: f64) -> Array1<Complex<f64>> {
        let dim = 1 << self.num_qubits;
        let mut cp = Array2::<Complex<f64>>::eye(dim);

        for i in 0..dim {
            let binary = format!("{:0width$b}", i, width = self.num_qubits);
            if binary.chars().nth(control).unwrap() == '1' && binary.chars().nth(target).unwrap() == '1' {
                cp[[i, i]] = Complex::from_polar(1.0, theta);
            }
        }

        cp.dot(state)
    }

    fn apply_swap(&self, state: &Array1<Complex<f64>>, qubit1: usize, qubit2: usize) -> Array1<Complex<f64>> {
        let dim = 1 << self.num_qubits;
        let mut swap = Array2::<Complex<f64>>::eye(dim);

        for i in 0..dim {
            let binary = format!("{:0width$b}", i, width = self.num_qubits);
            let mut swapped = binary.bytes().collect::<Vec<u8>>();
            swapped[qubit1] = binary.chars().nth(qubit2).unwrap() as u8;
            swapped[qubit2] = binary.chars().nth(qubit1).unwrap() as u8;
            let j = usize::from_str_radix(std::str::from_utf8(&swapped).unwrap(), 2).unwrap();
            swap[[i, i]] = Complex::new(0.0, 0.0);
            swap[[i, j]] = Complex::new(1.0, 0.0);
        }

        swap.dot(state)
    }
}

fn kron(a: ArrayView2<Complex<f64>>, b: ArrayView2<Complex<f64>>) -> Array2<Complex<f64>> {
    let (a_rows, a_cols) = a.dim();
    let (b_rows, b_cols) = b.dim();
    let mut result = Array2::<Complex<f64>>::zeros((a_rows * b_rows, a_cols * b_cols));

    for i in 0..a_rows {
        for j in 0..a_cols {
            for k in 0..b_rows {
                for l in 0..b_cols {
                    result[[i * b_rows + k, j * b_cols + l]] = a[[i, j]] * b[[k, l]];
                }
            }
        }
    }

    result
}