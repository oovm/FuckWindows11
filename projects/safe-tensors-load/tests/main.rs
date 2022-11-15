#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() {
    prost_build::compile_protos(&["../onnx/onnx/onnx.proto3"], &["../onnx/onnx"]).unwrap();
    std::fs::copy("foo.txt", "bar.txt")?;
}
