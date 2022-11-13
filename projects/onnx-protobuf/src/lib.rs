mod attrs;
mod util;

pub use self::attrs::*;
pub use self::util::*;

include!(concat!(env!("OUT_DIR"), "/onnx.rs"));

use self::{
    tensor_proto::DataType,
    tensor_shape_proto::{dimension, Dimension},
    type_proto::{Tensor, Value},
};

impl From<i64> for dimension::Value {
    fn from(v: i64) -> Self {
        dimension::Value::DimValue(v)
    }
}

impl From<String> for dimension::Value {
    fn from(v: String) -> Self {
        dimension::Value::DimParam(v)
    }
}

impl<T: Into<Dimension>> From<Vec<T>> for TensorShapeProto {
    fn from(v: Vec<T>) -> Self {
        TensorShapeProto {
            dim: v.into_iter().map(Into::into).collect(),
        }
    }
}

impl<V: Into<dimension::Value>> From<V> for Dimension {
    fn from(v: V) -> Self {
        tensor_shape_proto::Dimension {
            denotation: String::default(),
            value: Some(v.into()),
        }
    }
}

impl<K: Into<String>, V: Into<String>> From<(K, V)> for StringStringEntryProto {
    fn from((k, v): (K, V)) -> Self {
        StringStringEntryProto {
            key: k.into(),
            value: v.into(),
        }
    }
}

impl<T: Into<String>> From<T> for ValueInfoProto {
    fn from(name: T) -> Self {
        ValueInfoProto {
            name: name.into(),
            ..ValueInfoProto::default()
        }
    }
}

impl From<f32> for TensorProto {
    fn from(data: f32) -> TensorProto {
        TensorProto {
            dims: vec![1],
            float_data: vec![data],
            data_type: DataType::Float as i32,
            ..TensorProto::default()
        }
    }
}

impl From<Vec<f32>> for TensorProto {
    fn from(data: Vec<f32>) -> TensorProto {
        TensorProto {
            dims: vec![data.len() as i64],
            float_data: data,
            data_type: DataType::Float as i32,
            ..TensorProto::default()
        }
    }
}

impl From<i32> for TensorProto {
    fn from(data: i32) -> TensorProto {
        TensorProto {
            dims: vec![1],
            int32_data: vec![data],
            data_type: DataType::Int32 as i32,
            ..TensorProto::default()
        }
    }
}

impl From<Vec<i32>> for TensorProto {
    fn from(data: Vec<i32>) -> TensorProto {
        TensorProto {
            dims: vec![data.len() as i64],
            int32_data: data,
            data_type: DataType::Int32 as i32,
            ..TensorProto::default()
        }
    }
}

impl From<i64> for TensorProto {
    fn from(data: i64) -> TensorProto {
        TensorProto {
            dims: vec![1],
            int64_data: vec![data],
            data_type: DataType::Int64 as i32,
            ..TensorProto::default()
        }
    }
}

impl From<Vec<i64>> for TensorProto {
    fn from(data: Vec<i64>) -> TensorProto {
        TensorProto {
            dims: vec![data.len() as i64],
            int64_data: data,
            data_type: DataType::Int64 as i32,
            ..TensorProto::default()
        }
    }
}

impl From<f64> for TensorProto {
    fn from(data: f64) -> TensorProto {
        TensorProto {
            dims: vec![1],
            double_data: vec![data],
            data_type: DataType::Double as i32,
            ..TensorProto::default()
        }
    }
}

impl From<Vec<f64>> for TensorProto {
    fn from(data: Vec<f64>) -> TensorProto {
        TensorProto {
            dims: vec![data.len() as i64],
            double_data: data,
            data_type: DataType::Double as i32,
            ..TensorProto::default()
        }
    }
}

impl From<u64> for TensorProto {
    fn from(data: u64) -> TensorProto {
        TensorProto {
            dims: vec![1],
            uint64_data: vec![data],
            data_type: DataType::Uint64 as i32,
            ..TensorProto::default()
        }
    }
}

impl From<Vec<u64>> for TensorProto {
    fn from(data: Vec<u64>) -> TensorProto {
        TensorProto {
            dims: vec![data.len() as i64],
            uint64_data: data,
            data_type: DataType::Uint64 as i32,
            ..TensorProto::default()
        }
    }
}

impl From<DataType> for TypeProto {
    fn from(typ: DataType) -> TypeProto {
        TypeProto {
            denotation: "".to_owned(),
            value: Some(Value::TensorType(Tensor {
                elem_type: typ as i32,
                shape: None,
            })),
        }
    }
}
