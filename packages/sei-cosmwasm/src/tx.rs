// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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

//! Generated file from `tx.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MsgPlaceOrdersResponse)
pub struct MsgPlaceOrdersResponse {
    // message fields
    // @@protoc_insertion_point(field:MsgPlaceOrdersResponse.order_ids)
    pub order_ids: ::std::vec::Vec<u64>,
    // special fields
    // @@protoc_insertion_point(special_field:MsgPlaceOrdersResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgPlaceOrdersResponse {
    fn default() -> &'a MsgPlaceOrdersResponse {
        <MsgPlaceOrdersResponse as ::protobuf::Message>::default_instance()
    }
}

impl MsgPlaceOrdersResponse {
    pub fn new() -> MsgPlaceOrdersResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "order_ids",
            |m: &MsgPlaceOrdersResponse| { &m.order_ids },
            |m: &mut MsgPlaceOrdersResponse| { &mut m.order_ids },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgPlaceOrdersResponse>(
            "MsgPlaceOrdersResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgPlaceOrdersResponse {
    const NAME: &'static str = "MsgPlaceOrdersResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint64_into(&mut self.order_ids)?;
                },
                8 => {
                    self.order_ids.push(is.read_uint64()?);
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
        for value in &self.order_ids {
            my_size += ::protobuf::rt::uint64_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.order_ids {
            os.write_uint64(1, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgPlaceOrdersResponse {
        MsgPlaceOrdersResponse::new()
    }

    fn clear(&mut self) {
        self.order_ids.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgPlaceOrdersResponse {
        static instance: MsgPlaceOrdersResponse = MsgPlaceOrdersResponse {
            order_ids: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgPlaceOrdersResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgPlaceOrdersResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgPlaceOrdersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgPlaceOrdersResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08tx.proto\"5\n\x16MsgPlaceOrdersResponse\x12\x1b\n\torder_ids\x18\
    \x01\x20\x03(\x04R\x08orderIdsJo\n\x06\x12\x04\0\0\x04\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x04\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x02\x08\x1e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x02\x20\n\
    \x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\x03\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x12\x1b\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x1e\x1fb\x06proto3\
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
            messages.push(MsgPlaceOrdersResponse::generated_message_descriptor_data());
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
