# simulation-quantum-circuit
An example of simulating quantum circuits in different languages.
这是一个例子，用不同的语言进行模拟量子电路。

## 代码结构
* python：文件夹中包含了利用python构建的电路类型，以及各个电路的运行。
* rust：rust的实现与运行。
* julia：julia的实现与运行。
* C++：C++的实现与运行。

## 电路的模拟过程
目前电路的模拟过程，采用了是一层一层作用量子门，最终对所有比特进行量子态测量。其中CSS为17个量子比特的模拟器，通过上述模拟方法，无法有效进行模拟。
