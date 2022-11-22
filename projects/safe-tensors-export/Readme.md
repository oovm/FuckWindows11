# Convert weights to safe tensors

## Installation

```sh
cargo install safe-tensors-export
```

## Usage

```sh
safe-tensors-export -d models -p *.onnx,*.pb
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