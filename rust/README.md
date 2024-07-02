# Rust

## 环境描述
我使用的rust环境配置可以参考https://www.runoob.com/rust/rust-setup.html。

运行命令：
```
rustc -V 
rustc 1.67.1 (d5a82bbd2 2023-02-07)

cargo -V
cargo 1.67.1 (8ecd4f20a 2023-01-10)
```
这是我使用的rust版本。

## 代码结构

* main为主函数，其中实现了对电路的生成、模拟、测量。
* xxx_circuit为对应电路的生成函数，通过main进行调用。
* circuit.rs，为电路和指令的结构体和对应的函数。
* measure.rs，实现了对量子态的测量。

## 运行操作
```
cd rust
cargo build
cargo run
```