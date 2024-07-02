import numpy as np

from typing import List, Iterator

class Instruction:
    def __init__(self, name: str, qubits: List[int], parameter: float = None):
        self.name = name
        self.qubits = qubits
        self.parameter = parameter  # 用于CP门的相位参数

    def __repr__(self):
        return f"Instruction(name={self.name}, qubits={self.qubits}, parameter={self.parameter})"

class Circuit:
    def __init__(self, num_qubits: int):
        self.num_qubits = num_qubits
        self.instructions = []

    def add_instruction(self, instruction: Instruction):
        self.instructions.append(instruction)

    def simulate(self):
        # 初始量子态为 |0...0>
        state = np.zeros(2**self.num_qubits, dtype=complex)
        state[0] = 1.0  # |0...0>

        for instr in self.instructions:
            if instr.name == "H":
                state = self.apply_single_qubit_gate(state, self.hadamard_gate(), instr.qubits[0])
            elif instr.name == "X":
                state = self.apply_single_qubit_gate(state, self.x_gate(), instr.qubits[0])
            elif instr.name == "CNOT":
                state = self.apply_cnot(state, instr.qubits[0], instr.qubits[1])
            elif instr.name == "CP":
                state = self.apply_cp(state, instr.qubits[0], instr.qubits[1], instr.parameter)
            elif instr.name == "SWAP":
                state = self.apply_swap(state, instr.qubits[0], instr.qubits[1])
        
        return state

    def hadamard_gate(self):
        return (1 / np.sqrt(2)) * np.array([[1, 1], [1, -1]])

    def x_gate(self):
        return np.array([[0, 1], [1, 0]])

    def apply_single_qubit_gate(self, state, gate, qubit):
        full_gate = 1
        for i in range(self.num_qubits):
            if i == qubit:
                full_gate = np.kron(full_gate, gate)
            else:
                full_gate = np.kron(full_gate, np.eye(2))
        return full_gate @ state

    def apply_cnot(self, state, control, target):
        dim = 2 ** self.num_qubits
        CNOT = np.eye(dim, dtype=complex)
        
        for i in range(dim):
            binary = format(i, f'0{self.num_qubits}b')
            if binary[control] == '1':
                flipped = binary[:target] + ('0' if binary[target] == '1' else '1') + binary[target+1:]
                j = int(flipped, 2)
                CNOT[i, i] = 0
                CNOT[i, j] = 1

        return CNOT @ state

    def apply_cp(self, state, control, target, theta):
        dim = 2 ** self.num_qubits
        CP = np.eye(dim, dtype=complex)

        for i in range(dim):
            binary = format(i, f'0{self.num_qubits}b')
            if binary[control] == '1' and binary[target] == '1':
                CP[i, i] = np.exp(1j * theta)

        return CP @ state

    def apply_swap(self, state, qubit1, qubit2):
        dim = 2 ** self.num_qubits
        SWAP = np.eye(dim, dtype=complex)

        for i in range(dim):
            binary = format(i, f'0{self.num_qubits}b')
            swapped = list(binary)
            swapped[qubit1], swapped[qubit2] = swapped[qubit2], swapped[qubit1]
            j = int(''.join(swapped), 2)
            SWAP[i, i] = 0
            SWAP[i, j] = 1

        return SWAP @ state
    
    def __iter__(self) -> Iterator[Instruction]:
        return iter(self.instructions)

    def __repr__(self):
        return f"Circuit(instructions={self.instructions})"