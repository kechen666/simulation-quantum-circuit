import numpy as np

from circuit import Instruction, Circuit
from measure import measure_sample_all

def build_css_circuit(num_qubits):
    # Create a quantum circuit with n counting qubits plus one target qubit
    circuit = Circuit(num_qubits)
    data = range(9)
    mz = [9,10,11,12]
    mx = [13,14,15,16]
    # Top left
    circuit.add_instruction(Instruction(name="H", qubits=[mx[0]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[0], data[1]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[0], data[0]]))
    circuit.add_instruction(Instruction(name="H", qubits=[mx[0]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[1], mz[0]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[4], mz[0]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[0], mz[0]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[3], mz[0]]))
    
    # Top right
    circuit.add_instruction(Instruction(name="H", qubits=[mx[1]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[1], data[2]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[1], data[1]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[1], data[2]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[1], data[1]]))
    circuit.add_instruction(Instruction(name="H", qubits=[mx[1]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[2], mz[1]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[5], mz[1]]))
    
    # Bottom left
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[3], mz[2]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[6], mz[2]]))
    circuit.add_instruction(Instruction(name="H", qubits=[mx[2]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[2], data[4]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[2], data[3]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[2], data[7]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[2], data[6]]))
    circuit.add_instruction(Instruction(name="H", qubits=[mx[2]]))
    
    # Bottom right
    circuit.add_instruction(Instruction(name="H", qubits=[mx[3]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[3], data[8]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[mx[3], data[7]]))
    circuit.add_instruction(Instruction(name="H", qubits=[mx[3]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[5], mz[3]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[8], mz[3]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[4], mz[3]]))
    circuit.add_instruction(Instruction(name="CNOT", qubits=[data[7], mz[3]]))
    
    return circuit

if __name__=="__main__":
    # num_qubits = 17
    # qpe_circuit = build_css_circuit(num_qubits)

    # #打印电路指令
    # print("Circuit instruction is:")
    # for instr in qpe_circuit:
    #     print(instr)
        
    # final_state = qpe_circuit.simulate()

    # # 输出最终量子态
    # print("Final quantum state (CSS):")
    # print(final_state)
    
    # # 进行测量，实验中，我们没有实现局部测量，我们直接对全部进行了测量。
    # counts = measure_sample_all(final_state=final_state, num_qubits=num_qubits, shot=1024, seed = 100)
    # print("Result counts:")
    # print(counts)
    print("Can't ran in the circuit")
    