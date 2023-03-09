use safetensors::{Dtype, SafeTensorError};
use safetensors::tensor::TensorView;
use crate::protos::tensor::TensorProto;

use crate::TensorProto;

impl<'a, 'b> TryFrom<&'a TensorProto> for TensorView<'b>
    where 'a: 'b
{
    type Error = SafeTensorError;

    fn try_from(tensor: &'a TensorProto) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[inline(always)]
fn get_shape(tensor: &[i64]) -> Vec<usize> {
    tensor.iter().map(|x| *x as usize).collect()
}

#[inline(always)]
fn get_type(tensor: &Option<i32>) -> Dtype {
    match tensor {
        Some(1) => Dtype::F32,
        Some(n) => unimplemented!("Dtype {} not implemented", n),
        None => Dtype::F32,
    }
}