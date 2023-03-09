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

//! Generated file from `tensorflow/core/framework/graph.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:tensorflow.GraphDef)
pub struct GraphDef {
    // message fields
    // @@protoc_insertion_point(field:tensorflow.GraphDef.node)
    pub node: ::std::vec::Vec<super::node_def::NodeDef>,
    // @@protoc_insertion_point(field:tensorflow.GraphDef.versions)
    pub versions: ::protobuf::MessageField<super::versions::VersionDef>,
    // @@protoc_insertion_point(field:tensorflow.GraphDef.version)
    pub version: i32,
    // @@protoc_insertion_point(field:tensorflow.GraphDef.library)
    pub library: ::protobuf::MessageField<super::function::FunctionDefLibrary>,
    // special fields
    // @@protoc_insertion_point(special_field:tensorflow.GraphDef.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GraphDef {
    fn default() -> &'a GraphDef {
        <GraphDef as ::protobuf::Message>::default_instance()
    }
}

impl GraphDef {
    pub fn new() -> GraphDef {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "node",
            |m: &GraphDef| { &m.node },
            |m: &mut GraphDef| { &mut m.node },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::versions::VersionDef>(
            "versions",
            |m: &GraphDef| { &m.versions },
            |m: &mut GraphDef| { &mut m.versions },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &GraphDef| { &m.version },
            |m: &mut GraphDef| { &mut m.version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::function::FunctionDefLibrary>(
            "library",
            |m: &GraphDef| { &m.library },
            |m: &mut GraphDef| { &mut m.library },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GraphDef>(
            "GraphDef",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GraphDef {
    const NAME: &'static str = "GraphDef";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.node.push(is.read_message()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.versions)?;
                },
                24 => {
                    self.version = is.read_int32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.library)?;
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
        for value in &self.node {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.versions.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.version);
        }
        if let Some(v) = self.library.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.node {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.versions.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.version != 0 {
            os.write_int32(3, self.version)?;
        }
        if let Some(v) = self.library.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> GraphDef {
        GraphDef::new()
    }

    fn clear(&mut self) {
        self.node.clear();
        self.versions.clear();
        self.version = 0;
        self.library.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GraphDef {
        static instance: GraphDef = GraphDef {
            node: ::std::vec::Vec::new(),
            versions: ::protobuf::MessageField::none(),
            version: 0,
            library: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GraphDef {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GraphDef").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GraphDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphDef {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/framework/graph.proto\x12\ntensorflow\x1a(tensorflow/\
    core/framework/function.proto\x1a(tensorflow/core/framework/node_def.pro\
    to\x1a(tensorflow/core/framework/versions.proto\"\xbf\x01\n\x08GraphDef\
    \x12'\n\x04node\x18\x01\x20\x03(\x0b2\x13.tensorflow.NodeDefR\x04node\
    \x122\n\x08versions\x18\x04\x20\x01(\x0b2\x16.tensorflow.VersionDefR\x08\
    versions\x12\x1c\n\x07version\x18\x03\x20\x01(\x05R\x07versionB\x02\x18\
    \x01\x128\n\x07library\x18\x02\x20\x01(\x0b2\x1e.tensorflow.FunctionDefL\
    ibraryR\x07libraryBz\n\x18org.tensorflow.frameworkB\x0bGraphProtosP\x01Z\
    Lgithub.com/tensorflow/tensorflow/tensorflow/go/core/framework/graph_go_\
    proto\xf8\x01\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::function::file_descriptor().clone());
            deps.push(super::node_def::file_descriptor().clone());
            deps.push(super::versions::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GraphDef::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
