use std::collections::BTreeMap;
use std::path::Path;

use safetensors::{SafeTensorError, serialize_to_file};
use safetensors::tensor::TensorView;

use onnx_protobuf::load_model;


/// Convert an ONNX model to a SafeTensor weights.
///
/// # Arguments
///
/// * `input`: Path to the ONNX model.
/// * `output`: Path to the SafeTensor weights.
///
/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use safe_tensors_loader::convert_onnx;
/// convert_onnx(Path::new("tests/diffuser.onnx"), Path::new("tests/diffuser.safetensors")).unwrap();
/// ```
pub fn convert_onnx(input: &Path, output: &Path) -> Result<(), SafeTensorError> {
    let path = input.canonicalize()?;
    let mut map = BTreeMap::default();
    let graph = match load_model(path) {
        Ok(o) => { o.graph }
        Err(e) => {
            Err(SafeTensorError::InvalidOffset(e.to_string()))?
        }
    };
    for initializer in graph.initializer.iter() {
        let name = initializer.name().to_string();
        match map.get(&name) {
            Some(_) => {
                println!("{} already exists", name);
                continue;
            }
            None => {}
        }
        map.insert(name, TensorView::try_from(initializer)?);
    }
    serialize_to_file(map.iter(), &None, output)
}