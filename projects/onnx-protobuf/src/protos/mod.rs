// @generated

use std::fs::File;
use std::path::Path;

use protobuf::{Error, Message};

use crate::ModelProto;

pub mod onnx;

/// Load the model from a file.
pub fn load_model<P: AsRef<Path>>(path: P) -> Result<ModelProto, Error> {
    let mut file = File::open(path)?;
    let model = ModelProto::parse_from_reader(&mut file)?;
    Ok(model)
}
