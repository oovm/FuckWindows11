use std::path::PathBuf;

use protobuf_codegen::Codegen;

use onnx_protobuf::load_onnx;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn build_protobuf() -> std::io::Result<()> {
    let projects = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../").canonicalize()?;
    Codegen::new()
        .pure()
        .out_dir(projects.join("onnx-protobuf/src/protos"))
        .inputs(&[projects.join("onnx/onnx/onnx.proto")])
        .include(projects.join("onnx/onnx"))
        .run()
        .expect("Codegen failed.");
    Ok(())
}

#[test]
fn test_load() {
    let onnx = load_onnx("tests/noise0_model.onnx");
    println!("{:?}", onnx);
}