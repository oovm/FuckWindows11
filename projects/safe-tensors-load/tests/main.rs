use std::path::{Path, PathBuf};

use protobuf_codegen::Codegen;

use safe_tensors_loader::convert_onnx;

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

//_model = onnx.load("in.onnx")
// INTIALIZERS = _model.graph.initializer
// weights = {}
// for initializer in INTIALIZERS:
//     w = numpy_helper.to_array(initializer)
//     weights[initializer.name] = w
//
//
// save_file(weights, "out.safetensors")
#[test]
fn test_onnx() {
    convert_onnx(
        Path::new("tests/noise0_model.onnx"),
        Path::new("tests/noise0_model.safetensors"),
    ).unwrap();
}