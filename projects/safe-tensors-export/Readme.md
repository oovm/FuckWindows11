# Convert weights to safe tensors

## Installation

```sh
cargo install safe-tensors-export
```

## Usage

```sh
RUST_LOG=info safe-tensors-export -p "*.onnx,*.pb"
```

All parameters:

```yaml
Usage: safe-tensors-export [OPTIONS]

Options:
  -d, --directory <DIRECTORY>  
  -p, --pattern <PATTERN>
  -h, --help                   Print help
  -V, --version                Print version
```