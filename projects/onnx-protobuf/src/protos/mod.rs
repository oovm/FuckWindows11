// @generated

use std::fs::File;
use std::path::Path;

use protobuf::{Error, Message};

use onnx::TensorProto;

use crate::{AttributeProto, ModelProto};

pub mod onnx;

/// Load the model from a file.
pub fn load_model<P: AsRef<Path>>(path: P) -> Result<ModelProto, Error> {
    let mut file = File::open(path)?;
    let model = ModelProto::parse_from_reader(&mut file)?;
    Ok(model)
}

/// Load the attribute from a file.
pub fn load_attribute<P: AsRef<Path>>(path: P) -> Result<AttributeProto, Error> {
    let mut file = File::open(path)?;
    let attr = AttributeProto::parse_from_reader(&mut file)?;
    Ok(attr)
}