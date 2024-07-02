import numpy as np

from circuit import Instruction, Circuit
from measure import measure_sample_all

def build_ghz_circuit(num_qubits = 3):
    circuit = Circuit(num_qubits)

    # 应用Hadamard门
    circuit.add_instruction(Instruction(name="H", qubits=[0]))

    # 应用CNOT门
    circuit.add_instruction(Instruction(name="CNOT", qubits=[0,1]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[0,2]))

    return circuit

if __name__=="__main__":
    num_qubits = 3
    ghz_circuit = build_ghz_circuit(num_qubits)

    #打印电路指令
    print("Circuit instruction is:")
    for instr in ghz_circuit:
        print(instr)
        
    final_state = ghz_circuit.simulate()

    # 输出最终量子态
    print("Final quantum state (GHZ):")
    print(final_state)
    
    # 进行测量
    counts = measure_sample_all(final_state=final_state, num_qubits=num_qubits, shot=1024, seed = 50)
    print("Result counts:")
    print(counts)