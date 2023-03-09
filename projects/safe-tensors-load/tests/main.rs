use std::path::PathBuf;

use protobuf_codegen::Codegen;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn build_onnx_proto() -> std::io::Result<()> {
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
#[ignore]
fn build_tensorflow_proto() -> std::io::Result<()> {
    let projects = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../").canonicalize()?;
    Codegen::new()
        .pure()
        .out_dir(projects.join("tensorflow-protobuf/src/protos"))
        .inputs(&[projects.join("tensorflow/tensorflow/core/framework/function.proto")])
        .include(projects.join("tensorflow/tensorflow/"))
        .run()
        .expect("Codegen failed.");
    Ok(())
}