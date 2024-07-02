use ndarray::ArrayBase;
use ndarray::OwnedRepr;
use ndarray::Dim;
use rand::prelude::*;
use std::collections::HashMap;

/// 计算测量概率并返回测量结果的二进制表示
pub fn measure_state_all(state: &ArrayBase<OwnedRepr<num_complex::Complex<f64>>, Dim<[usize; 1]>>, num_qubits: usize, rng: &mut dyn RngCore) -> String {
    // 计算测量概率
    let probabilities: Vec<f64> = state.iter().map(|&amplitude| amplitude.norm_sqr()).collect();
    let rand_num: f64 = rng.gen();

    let mut cumulative_prob = 0.0;
    for (idx, &prob) in probabilities.iter().enumerate() {
        cumulative_prob += prob;
        if rand_num < cumulative_prob {
            return format!("{:0width$b}", idx, width = num_qubits);
        }
    }

    "0".repeat(num_qubits)
}

/// 对最终量子态进行多次采样测量
pub fn measure_sample_all(final_state: &ArrayBase<OwnedRepr<num_complex::Complex<f64>>, Dim<[usize; 1]>>, num_qubits: usize, shot: usize, seed: u64) -> HashMap<String, usize> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut counts = HashMap::new();

    for _ in 0..shot {
        let m = measure_state_all(final_state, num_qubits, &mut rng);
        *counts.entry(m).or_insert(0) += 1;
    }

    counts
}
