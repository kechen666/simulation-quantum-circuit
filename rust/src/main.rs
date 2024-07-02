mod circuit;

mod bell_circuit; // 导入额外的 bell_circuit.rs 模块
mod ghz_circuit; // 导入 ghz_circuit.rs 模块
mod qft_circuit; // 导入 qft_circuit.rs 模块
mod qpe_circuit; // 导入 qpe_circuit.rs 模块

mod measure;

use bell_circuit::create_bell_circuit; // 导入创建贝尔态电路的函数
use ghz_circuit::create_ghz_circuit; // 导入创建 GHZ 状态电路的函数
use qft_circuit::create_qft_circuit; // 导入创建 QFT 电路的函数
use qpe_circuit::create_qpe_circuit; // 导入创建 QPE 电路的函数


use measure::measure_sample_all;

fn main() {
    let bell_circuit: circuit::Circuit = create_bell_circuit();

    println!("Circuit instructions: {:?}", bell_circuit.instructions);

    let bell_final_state = bell_circuit.simulate();
    println!("Bell Final state: {:?}", bell_final_state);

    println!("---------------------------");


    // 测量Bell
    let num_qubits = 2;
    // let final_state: ArrayBase<OwnedRepr<num_complex::Complex<f64>>, Dim<[usize; 1]>> = vec![1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let shot = 1024;
    let seed = 42;

    let counts = measure_sample_all(&bell_final_state, num_qubits, shot, seed);

    println!("Result counts:");
    for (state, count) in counts.iter() {
        println!("{}: {}", state, count);
    }

    // 创建 GHZ 状态电路示例（3 比特）
    let ghz_circuit = create_ghz_circuit(3);

    println!("GHZ Circuit instructions: {:?}", ghz_circuit.instructions);

    let ghz_final_state = ghz_circuit.simulate();
    println!("GHZ Final state: {:?}", ghz_final_state);

    println!("---------------------------");

    // 测量GHZ
    let num_qubits = 3;
    // let final_state: ArrayBase<OwnedRepr<num_complex::Complex<f64>>, Dim<[usize; 1]>> = vec![1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let shot = 1024;
    let seed = 42;

    let counts = measure_sample_all(&ghz_final_state, num_qubits, shot, seed);

    println!("Result counts:");
    for (state, count) in counts.iter() {
        println!("{}: {}", state, count);
    }

    // 创建 5 比特的量子傅里叶变换电路示例
    let qft_circuit = create_qft_circuit(5);

    println!("QFT Circuit instructions: {:?}", qft_circuit.instructions);

    let qft_final_state = qft_circuit.simulate();
    println!("QFT Final state: {:?}", qft_final_state);

    // 测量QFT
    let num_qubits = 5;
    // let final_state: ArrayBase<OwnedRepr<num_complex::Complex<f64>>, Dim<[usize; 1]>> = vec![1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let shot = 1024;
    let seed = 42;

    let counts = measure_sample_all(&qft_final_state, num_qubits, shot, seed);

    println!("Result counts:");
    for (state, count) in counts.iter() {
        println!("{}: {}", state, count);
    }

    println!("---------------------------");

    // 创建 5 比特的量子相位估计电路示例
    let qpe_circuit = create_qpe_circuit(5);

    println!("QPE Circuit instructions: {:?}", qpe_circuit.instructions);

    let qpe_final_state: ndarray::ArrayBase<ndarray::OwnedRepr<num_complex::Complex<f64>>, ndarray::Dim<[usize; 1]>> = qpe_circuit.simulate();
    println!("QPE Final state: {:?}", qpe_final_state);

    // 测量QPE
    let num_qubits = 5;
    // let final_state: ArrayBase<OwnedRepr<num_complex::Complex<f64>>, Dim<[usize; 1]>> = vec![1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let shot = 1024;
    let seed = 42;

    let counts = measure_sample_all(&qpe_final_state, num_qubits+1, shot, seed);

    println!("Result counts:");
    for (state, count) in counts.iter() {
        println!("{}: {}", state, count);
    }


}
