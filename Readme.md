Safe Tensors Export
===================

## Install

```sh
cargo install safe-tensors-export
```

## Usage

```sh
# Convert onnx weights to safe tensors
RUST_LOG=info safe-tensors-export -p "*.onnx,*.pb"
```