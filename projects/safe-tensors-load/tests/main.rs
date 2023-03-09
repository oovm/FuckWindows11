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
    let tensorflow = projects.join("tensorflow/tensorflow").canonicalize()?;
    Codegen::new()
        .pure()
        .out_dir(projects.join("tensorflow-protobuf/src/protos"))
        .inputs(&[
            tensorflow.join("core/framework/graph.proto"),
            tensorflow.join("core/framework/node_def.proto"),
            tensorflow.join("core/framework/versions.proto"),
            tensorflow.join("core/framework/function.proto"),
            tensorflow.join("core/framework/op_def.proto"),
            tensorflow.join("core/framework/attr_value.proto"),
            tensorflow.join("core/framework/types.proto"),
            tensorflow.join("core/framework/full_type.proto"),
            tensorflow.join("core/framework/resource_handle.proto"),
            //
            tensorflow.join("core/framework/tensor.proto"),
            tensorflow.join("core/framework/tensor_shape.proto"),
        ])
        .includes(&[
            projects.join("tensorflow"),
        ])
        .run()
        .expect("Codegen failed.");
    Ok(())
}