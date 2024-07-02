import numpy as np

from circuit import Instruction, Circuit
from measure import measure_sample_all

def build_qft_circuit(num_qubits):
    circuit = Circuit(num_qubits)

    # 应用Hadamard门
    for qubit in range(num_qubits):
        circuit.add_instruction(Instruction(name="H", qubits=[qubit]))

    # 应用控制相位门
    for control in range(num_qubits):
        circuit.add_instruction(Instruction(name="H", qubits=[control]))
        for target in range(control + 1, num_qubits):
            theta = 2 * np.pi / (2 ** (target - control+1))
            circuit.add_instruction(Instruction(name="CP", qubits=[control, target], parameter=theta))

    # 反序SWAP门
    for qubit in range(num_qubits // 2):
        circuit.add_instruction(Instruction(name="SWAP", qubits=[qubit, num_qubits - qubit - 1]))

    return circuit

if __name__=="__main__":
    num_qubits = 5
    qft_circuit = build_qft_circuit(num_qubits)

    #打印电路指令
    print("Circuit instruction is:")
    for instr in qft_circuit:
        print(instr)
        
    final_state = qft_circuit.simulate()

    # 输出最终量子态
    print("Final quantum state (QFT):")
    print(final_state)
    
    # 进行测量
    counts = measure_sample_all(final_state=final_state, num_qubits=num_qubits, shot=1024, seed = 100)
    print("Result counts:")
    print(counts)