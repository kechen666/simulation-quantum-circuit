import numpy as np
from numpy.random import default_rng
from typing import List

def measure_state_all(state, num_qubits, rng = None):
    # 计算测量概率
    probabilities = np.abs(state)**2
    # 生成随机数
    if rng == None:
        rng = default_rng()
    rand_num = rng.random()
    # 执行测量
    cumulative_prob = 0.0
    for idx, prob in enumerate(probabilities):
        cumulative_prob += prob
        if rand_num < cumulative_prob:
            # measured_state = np.zeros_like(state)
            # measured_state[idx] = 1.0
            measured_state = format(idx, f'0{num_qubits}b')
            return measured_state

def measure_sample_all(final_state, num_qubits: int, shot:int = 1024, seed = None):
    rng = default_rng(seed=seed)  # 使用默认的随机数生成器，并设置随机种子
    counts = {}
    for _ in range(shot):
        m = measure_state_all(final_state, num_qubits, rng)
        counts[m] = counts.get(m,0) + 1
    return counts

# 局部测量
def measure_sample(final_state, num_qubits: int, qubits_to_measure:List[int], shot:int = 1024, seed = None):
    dimension = 2**num_qubits
    
    num_qubits_to_measure = len(qubits_to_measure)
    
    # 生成测量矩阵
    measurement_matrix = np.zeros((2**num_qubits_to_measure, dimension))
    for i in range(2**num_qubits_to_measure):
        basis_state = np.binary_repr(i, width=num_qubits_to_measure)
        basis_index = sum([int(b) * 2**(num_qubits_to_measure - j - 1) for j, b in enumerate(basis_state)])
        for j, qubit in enumerate(qubits_to_measure):
            basis_index += int(basis_state[j]) * 2**qubit
        measurement_matrix[i, basis_index] = 1.0
    
    # 得到局部的测量状态
    measured_state = measurement_matrix @ final_state
    
    # 归一化测量态
    normalized_measured_state = measured_state / np.linalg.norm(measured_state)
    print("measured_state:", measured_state)
    probability_sum = np.sum(np.abs(measured_state)**2)
    print("Total probability after measurement:", probability_sum)
    counts = measure_sample_all(normalized_measured_state, num_qubits_to_measure, shot, seed)
    
    return counts