fn main() {
    prost_build::compile_protos(&["onnx/onnx.proto3"], &["third_party/onnx"]).unwrap();
}
