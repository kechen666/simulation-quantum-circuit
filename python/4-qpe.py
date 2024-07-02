import numpy as np

from circuit import Instruction, Circuit
from measure import measure_sample_all

def build_qpe_circuit(num_qubits):
    # Create a quantum circuit with n counting qubits plus one target qubit
    circuit = Circuit(num_qubits+1)

    # 目标比特应用H门
    circuit.add_instruction(Instruction(name="H", qubits=[num_qubits]))
    
    #应用H门
    for qubit in range(num_qubits):
        circuit.add_instruction(Instruction(name="H", qubits=[qubit]))

    # Apply controlled-U operations
    for control in range(num_qubits):
        repetitions = 2 ** control
        for _ in range(repetitions):
            theta = 2 * np.pi * 0.25 # Here, we assume θ = 0.25
            circuit.add_instruction(Instruction(name="CP", qubits=[control, num_qubits], parameter=theta))
    
    # # 反序SWAP门
    for qubit in range(num_qubits // 2):
        circuit.add_instruction(Instruction(name="SWAP", qubits=[qubit, num_qubits - qubit - 1]))
    
    # 逆QFT
    for qubit in range(num_qubits):
        for k in range(qubit):
            theta = -np.pi / (2 ** (qubit - k))
            circuit.add_instruction(Instruction(name="CP", qubits=[k, qubit], parameter=theta))
        circuit.add_instruction(Instruction(name="H", qubits=[qubit]))


    return circuit

if __name__=="__main__":
    num_qubits = 5
    qpe_circuit = build_qpe_circuit(num_qubits)

    #打印电路指令
    print("Circuit instruction is:")
    for instr in qpe_circuit:
        print(instr)
        
    final_state = qpe_circuit.simulate()

    # 输出最终量子态
    print("Final quantum state (QPE):")
    print(final_state)
    
    # 进行测量，实验中，我们没有实现局部测量，我们直接对全部进行了测量。
    counts = measure_sample_all(final_state=final_state, num_qubits=num_qubits+1, shot=1024, seed = 100)
    # counts = measure_sample(final_state=final_state, num_qubits=num_qubits+1, qubits_to_measure=[0,1,2,3,4], shot=1024, seed = 100)
    print("Result counts:")
    print(counts)
    
    # 000101对应的，012345的数据比特。这与qiskit不同，qiskit是从右向左。即第四个比特出现了1。
    # 01000(bin) = 8/2^5 = 1/4