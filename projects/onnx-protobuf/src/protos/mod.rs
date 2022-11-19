// @generated

use std::io::Read;
use std::path::Path;

use protobuf::Message;

pub mod onnx;


pub fn load_onnx<P: AsRef<Path>>(path: P) {
    let mut file = std::fs::File::open(path).unwrap();
    let model = onnx::ModelProto::parse_from_reader(&mut file).unwrap();
    println!("{:?}", model);
}