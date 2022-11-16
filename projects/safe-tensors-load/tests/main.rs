use std::path::PathBuf;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn build_protobuf() -> std::io::Result<()> {
    let projects = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../").canonicalize()?;
    protobuf_codegen_pure::Codegen::new()
        .out_dir(projects.join("onnx-protobuf/src/protos"))
        .inputs(&[projects.join("onnx/onnx/onnx.proto")])
        .include(projects.join("onnx/onnx"))
        .run()
        .expect("Codegen failed.");
    Ok(())
}
