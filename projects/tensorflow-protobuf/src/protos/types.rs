// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `tensorflow/core/framework/types.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tensorflow.SerializedDType)
pub struct SerializedDType {
    // message fields
    // @@protoc_insertion_point(field:tensorflow.SerializedDType.datatype)
    pub datatype: ::protobuf::EnumOrUnknown<DataType>,
    // special fields
    // @@protoc_insertion_point(special_field:tensorflow.SerializedDType.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SerializedDType {
    fn default() -> &'a SerializedDType {
        <SerializedDType as ::protobuf::Message>::default_instance()
    }
}

impl SerializedDType {
    pub fn new() -> SerializedDType {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "datatype",
            |m: &SerializedDType| { &m.datatype },
            |m: &mut SerializedDType| { &mut m.datatype },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SerializedDType>(
            "SerializedDType",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SerializedDType {
    const NAME: &'static str = "SerializedDType";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.datatype = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.datatype != ::protobuf::EnumOrUnknown::new(DataType::DT_INVALID) {
            my_size += ::protobuf::rt::int32_size(1, self.datatype.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.datatype != ::protobuf::EnumOrUnknown::new(DataType::DT_INVALID) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.datatype))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SerializedDType {
        SerializedDType::new()
    }

    fn clear(&mut self) {
        self.datatype = ::protobuf::EnumOrUnknown::new(DataType::DT_INVALID);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SerializedDType {
        static instance: SerializedDType = SerializedDType {
            datatype: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SerializedDType {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SerializedDType").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SerializedDType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializedDType {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:tensorflow.DataType)
pub enum DataType {
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INVALID)
    DT_INVALID = 0,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_FLOAT)
    DT_FLOAT = 1,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_DOUBLE)
    DT_DOUBLE = 2,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT32)
    DT_INT32 = 3,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT8)
    DT_UINT8 = 4,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT16)
    DT_INT16 = 5,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT8)
    DT_INT8 = 6,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_STRING)
    DT_STRING = 7,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_COMPLEX64)
    DT_COMPLEX64 = 8,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT64)
    DT_INT64 = 9,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_BOOL)
    DT_BOOL = 10,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QINT8)
    DT_QINT8 = 11,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QUINT8)
    DT_QUINT8 = 12,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QINT32)
    DT_QINT32 = 13,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_BFLOAT16)
    DT_BFLOAT16 = 14,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QINT16)
    DT_QINT16 = 15,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QUINT16)
    DT_QUINT16 = 16,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT16)
    DT_UINT16 = 17,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_COMPLEX128)
    DT_COMPLEX128 = 18,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_HALF)
    DT_HALF = 19,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_RESOURCE)
    DT_RESOURCE = 20,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_VARIANT)
    DT_VARIANT = 21,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT32)
    DT_UINT32 = 22,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT64)
    DT_UINT64 = 23,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_FLOAT8_E5M2)
    DT_FLOAT8_E5M2 = 24,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_FLOAT8_E4M3FN)
    DT_FLOAT8_E4M3FN = 25,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_FLOAT_REF)
    DT_FLOAT_REF = 101,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_DOUBLE_REF)
    DT_DOUBLE_REF = 102,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT32_REF)
    DT_INT32_REF = 103,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT8_REF)
    DT_UINT8_REF = 104,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT16_REF)
    DT_INT16_REF = 105,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT8_REF)
    DT_INT8_REF = 106,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_STRING_REF)
    DT_STRING_REF = 107,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_COMPLEX64_REF)
    DT_COMPLEX64_REF = 108,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_INT64_REF)
    DT_INT64_REF = 109,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_BOOL_REF)
    DT_BOOL_REF = 110,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QINT8_REF)
    DT_QINT8_REF = 111,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QUINT8_REF)
    DT_QUINT8_REF = 112,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QINT32_REF)
    DT_QINT32_REF = 113,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_BFLOAT16_REF)
    DT_BFLOAT16_REF = 114,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QINT16_REF)
    DT_QINT16_REF = 115,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_QUINT16_REF)
    DT_QUINT16_REF = 116,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT16_REF)
    DT_UINT16_REF = 117,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_COMPLEX128_REF)
    DT_COMPLEX128_REF = 118,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_HALF_REF)
    DT_HALF_REF = 119,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_RESOURCE_REF)
    DT_RESOURCE_REF = 120,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_VARIANT_REF)
    DT_VARIANT_REF = 121,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT32_REF)
    DT_UINT32_REF = 122,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_UINT64_REF)
    DT_UINT64_REF = 123,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_FLOAT8_E5M2_REF)
    DT_FLOAT8_E5M2_REF = 124,
    // @@protoc_insertion_point(enum_value:tensorflow.DataType.DT_FLOAT8_E4M3FN_REF)
    DT_FLOAT8_E4M3FN_REF = 125,
}

impl ::protobuf::Enum for DataType {
    const NAME: &'static str = "DataType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataType> {
        match value {
            0 => ::std::option::Option::Some(DataType::DT_INVALID),
            1 => ::std::option::Option::Some(DataType::DT_FLOAT),
            2 => ::std::option::Option::Some(DataType::DT_DOUBLE),
            3 => ::std::option::Option::Some(DataType::DT_INT32),
            4 => ::std::option::Option::Some(DataType::DT_UINT8),
            5 => ::std::option::Option::Some(DataType::DT_INT16),
            6 => ::std::option::Option::Some(DataType::DT_INT8),
            7 => ::std::option::Option::Some(DataType::DT_STRING),
            8 => ::std::option::Option::Some(DataType::DT_COMPLEX64),
            9 => ::std::option::Option::Some(DataType::DT_INT64),
            10 => ::std::option::Option::Some(DataType::DT_BOOL),
            11 => ::std::option::Option::Some(DataType::DT_QINT8),
            12 => ::std::option::Option::Some(DataType::DT_QUINT8),
            13 => ::std::option::Option::Some(DataType::DT_QINT32),
            14 => ::std::option::Option::Some(DataType::DT_BFLOAT16),
            15 => ::std::option::Option::Some(DataType::DT_QINT16),
            16 => ::std::option::Option::Some(DataType::DT_QUINT16),
            17 => ::std::option::Option::Some(DataType::DT_UINT16),
            18 => ::std::option::Option::Some(DataType::DT_COMPLEX128),
            19 => ::std::option::Option::Some(DataType::DT_HALF),
            20 => ::std::option::Option::Some(DataType::DT_RESOURCE),
            21 => ::std::option::Option::Some(DataType::DT_VARIANT),
            22 => ::std::option::Option::Some(DataType::DT_UINT32),
            23 => ::std::option::Option::Some(DataType::DT_UINT64),
            24 => ::std::option::Option::Some(DataType::DT_FLOAT8_E5M2),
            25 => ::std::option::Option::Some(DataType::DT_FLOAT8_E4M3FN),
            101 => ::std::option::Option::Some(DataType::DT_FLOAT_REF),
            102 => ::std::option::Option::Some(DataType::DT_DOUBLE_REF),
            103 => ::std::option::Option::Some(DataType::DT_INT32_REF),
            104 => ::std::option::Option::Some(DataType::DT_UINT8_REF),
            105 => ::std::option::Option::Some(DataType::DT_INT16_REF),
            106 => ::std::option::Option::Some(DataType::DT_INT8_REF),
            107 => ::std::option::Option::Some(DataType::DT_STRING_REF),
            108 => ::std::option::Option::Some(DataType::DT_COMPLEX64_REF),
            109 => ::std::option::Option::Some(DataType::DT_INT64_REF),
            110 => ::std::option::Option::Some(DataType::DT_BOOL_REF),
            111 => ::std::option::Option::Some(DataType::DT_QINT8_REF),
            112 => ::std::option::Option::Some(DataType::DT_QUINT8_REF),
            113 => ::std::option::Option::Some(DataType::DT_QINT32_REF),
            114 => ::std::option::Option::Some(DataType::DT_BFLOAT16_REF),
            115 => ::std::option::Option::Some(DataType::DT_QINT16_REF),
            116 => ::std::option::Option::Some(DataType::DT_QUINT16_REF),
            117 => ::std::option::Option::Some(DataType::DT_UINT16_REF),
            118 => ::std::option::Option::Some(DataType::DT_COMPLEX128_REF),
            119 => ::std::option::Option::Some(DataType::DT_HALF_REF),
            120 => ::std::option::Option::Some(DataType::DT_RESOURCE_REF),
            121 => ::std::option::Option::Some(DataType::DT_VARIANT_REF),
            122 => ::std::option::Option::Some(DataType::DT_UINT32_REF),
            123 => ::std::option::Option::Some(DataType::DT_UINT64_REF),
            124 => ::std::option::Option::Some(DataType::DT_FLOAT8_E5M2_REF),
            125 => ::std::option::Option::Some(DataType::DT_FLOAT8_E4M3FN_REF),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DataType] = &[
        DataType::DT_INVALID,
        DataType::DT_FLOAT,
        DataType::DT_DOUBLE,
        DataType::DT_INT32,
        DataType::DT_UINT8,
        DataType::DT_INT16,
        DataType::DT_INT8,
        DataType::DT_STRING,
        DataType::DT_COMPLEX64,
        DataType::DT_INT64,
        DataType::DT_BOOL,
        DataType::DT_QINT8,
        DataType::DT_QUINT8,
        DataType::DT_QINT32,
        DataType::DT_BFLOAT16,
        DataType::DT_QINT16,
        DataType::DT_QUINT16,
        DataType::DT_UINT16,
        DataType::DT_COMPLEX128,
        DataType::DT_HALF,
        DataType::DT_RESOURCE,
        DataType::DT_VARIANT,
        DataType::DT_UINT32,
        DataType::DT_UINT64,
        DataType::DT_FLOAT8_E5M2,
        DataType::DT_FLOAT8_E4M3FN,
        DataType::DT_FLOAT_REF,
        DataType::DT_DOUBLE_REF,
        DataType::DT_INT32_REF,
        DataType::DT_UINT8_REF,
        DataType::DT_INT16_REF,
        DataType::DT_INT8_REF,
        DataType::DT_STRING_REF,
        DataType::DT_COMPLEX64_REF,
        DataType::DT_INT64_REF,
        DataType::DT_BOOL_REF,
        DataType::DT_QINT8_REF,
        DataType::DT_QUINT8_REF,
        DataType::DT_QINT32_REF,
        DataType::DT_BFLOAT16_REF,
        DataType::DT_QINT16_REF,
        DataType::DT_QUINT16_REF,
        DataType::DT_UINT16_REF,
        DataType::DT_COMPLEX128_REF,
        DataType::DT_HALF_REF,
        DataType::DT_RESOURCE_REF,
        DataType::DT_VARIANT_REF,
        DataType::DT_UINT32_REF,
        DataType::DT_UINT64_REF,
        DataType::DT_FLOAT8_E5M2_REF,
        DataType::DT_FLOAT8_E4M3FN_REF,
    ];
}

impl ::protobuf::EnumFull for DataType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DataType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            DataType::DT_INVALID => 0,
            DataType::DT_FLOAT => 1,
            DataType::DT_DOUBLE => 2,
            DataType::DT_INT32 => 3,
            DataType::DT_UINT8 => 4,
            DataType::DT_INT16 => 5,
            DataType::DT_INT8 => 6,
            DataType::DT_STRING => 7,
            DataType::DT_COMPLEX64 => 8,
            DataType::DT_INT64 => 9,
            DataType::DT_BOOL => 10,
            DataType::DT_QINT8 => 11,
            DataType::DT_QUINT8 => 12,
            DataType::DT_QINT32 => 13,
            DataType::DT_BFLOAT16 => 14,
            DataType::DT_QINT16 => 15,
            DataType::DT_QUINT16 => 16,
            DataType::DT_UINT16 => 17,
            DataType::DT_COMPLEX128 => 18,
            DataType::DT_HALF => 19,
            DataType::DT_RESOURCE => 20,
            DataType::DT_VARIANT => 21,
            DataType::DT_UINT32 => 22,
            DataType::DT_UINT64 => 23,
            DataType::DT_FLOAT8_E5M2 => 24,
            DataType::DT_FLOAT8_E4M3FN => 25,
            DataType::DT_FLOAT_REF => 26,
            DataType::DT_DOUBLE_REF => 27,
            DataType::DT_INT32_REF => 28,
            DataType::DT_UINT8_REF => 29,
            DataType::DT_INT16_REF => 30,
            DataType::DT_INT8_REF => 31,
            DataType::DT_STRING_REF => 32,
            DataType::DT_COMPLEX64_REF => 33,
            DataType::DT_INT64_REF => 34,
            DataType::DT_BOOL_REF => 35,
            DataType::DT_QINT8_REF => 36,
            DataType::DT_QUINT8_REF => 37,
            DataType::DT_QINT32_REF => 38,
            DataType::DT_BFLOAT16_REF => 39,
            DataType::DT_QINT16_REF => 40,
            DataType::DT_QUINT16_REF => 41,
            DataType::DT_UINT16_REF => 42,
            DataType::DT_COMPLEX128_REF => 43,
            DataType::DT_HALF_REF => 44,
            DataType::DT_RESOURCE_REF => 45,
            DataType::DT_VARIANT_REF => 46,
            DataType::DT_UINT32_REF => 47,
            DataType::DT_UINT64_REF => 48,
            DataType::DT_FLOAT8_E5M2_REF => 49,
            DataType::DT_FLOAT8_E4M3FN_REF => 50,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DataType {
    fn default() -> Self {
        DataType::DT_INVALID
    }
}

impl DataType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DataType>("DataType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/framework/types.proto\x12\ntensorflow\"C\n\x0fSeriali\
    zedDType\x120\n\x08datatype\x18\x01\x20\x01(\x0e2\x14.tensorflow.DataTyp\
    eR\x08datatype*\x86\x07\n\x08DataType\x12\x0e\n\nDT_INVALID\x10\0\x12\
    \x0c\n\x08DT_FLOAT\x10\x01\x12\r\n\tDT_DOUBLE\x10\x02\x12\x0c\n\x08DT_IN\
    T32\x10\x03\x12\x0c\n\x08DT_UINT8\x10\x04\x12\x0c\n\x08DT_INT16\x10\x05\
    \x12\x0b\n\x07DT_INT8\x10\x06\x12\r\n\tDT_STRING\x10\x07\x12\x10\n\x0cDT\
    _COMPLEX64\x10\x08\x12\x0c\n\x08DT_INT64\x10\t\x12\x0b\n\x07DT_BOOL\x10\
    \n\x12\x0c\n\x08DT_QINT8\x10\x0b\x12\r\n\tDT_QUINT8\x10\x0c\x12\r\n\tDT_\
    QINT32\x10\r\x12\x0f\n\x0bDT_BFLOAT16\x10\x0e\x12\r\n\tDT_QINT16\x10\x0f\
    \x12\x0e\n\nDT_QUINT16\x10\x10\x12\r\n\tDT_UINT16\x10\x11\x12\x11\n\rDT_\
    COMPLEX128\x10\x12\x12\x0b\n\x07DT_HALF\x10\x13\x12\x0f\n\x0bDT_RESOURCE\
    \x10\x14\x12\x0e\n\nDT_VARIANT\x10\x15\x12\r\n\tDT_UINT32\x10\x16\x12\r\
    \n\tDT_UINT64\x10\x17\x12\x12\n\x0eDT_FLOAT8_E5M2\x10\x18\x12\x14\n\x10D\
    T_FLOAT8_E4M3FN\x10\x19\x12\x10\n\x0cDT_FLOAT_REF\x10e\x12\x11\n\rDT_DOU\
    BLE_REF\x10f\x12\x10\n\x0cDT_INT32_REF\x10g\x12\x10\n\x0cDT_UINT8_REF\
    \x10h\x12\x10\n\x0cDT_INT16_REF\x10i\x12\x0f\n\x0bDT_INT8_REF\x10j\x12\
    \x11\n\rDT_STRING_REF\x10k\x12\x14\n\x10DT_COMPLEX64_REF\x10l\x12\x10\n\
    \x0cDT_INT64_REF\x10m\x12\x0f\n\x0bDT_BOOL_REF\x10n\x12\x10\n\x0cDT_QINT\
    8_REF\x10o\x12\x11\n\rDT_QUINT8_REF\x10p\x12\x11\n\rDT_QINT32_REF\x10q\
    \x12\x13\n\x0fDT_BFLOAT16_REF\x10r\x12\x11\n\rDT_QINT16_REF\x10s\x12\x12\
    \n\x0eDT_QUINT16_REF\x10t\x12\x11\n\rDT_UINT16_REF\x10u\x12\x15\n\x11DT_\
    COMPLEX128_REF\x10v\x12\x0f\n\x0bDT_HALF_REF\x10w\x12\x13\n\x0fDT_RESOUR\
    CE_REF\x10x\x12\x12\n\x0eDT_VARIANT_REF\x10y\x12\x11\n\rDT_UINT32_REF\
    \x10z\x12\x11\n\rDT_UINT64_REF\x10{\x12\x16\n\x12DT_FLOAT8_E5M2_REF\x10|\
    \x12\x18\n\x14DT_FLOAT8_E4M3FN_REF\x10}Bz\n\x18org.tensorflow.frameworkB\
    \x0bTypesProtosP\x01ZLgithub.com/tensorflow/tensorflow/tensorflow/go/cor\
    e/framework/types_go_proto\xf8\x01\x01b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SerializedDType::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(DataType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
