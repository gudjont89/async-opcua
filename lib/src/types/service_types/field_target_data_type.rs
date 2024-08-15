// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTargetDataType {
    pub data_set_field_id: crate::types::guid::Guid,
    pub receiver_index_range: crate::types::string::UAString,
    pub target_node_id: crate::types::node_id::NodeId,
    pub attribute_id: u32,
    pub write_index_range: crate::types::string::UAString,
    pub override_value_handling: super::enums::OverrideValueHandling,
    pub override_value: crate::types::variant::Variant,
}
impl crate::types::MessageInfo for FieldTargetDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::FieldTargetDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for FieldTargetDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.data_set_field_id.byte_len();
        size += self.receiver_index_range.byte_len();
        size += self.target_node_id.byte_len();
        size += self.attribute_id.byte_len();
        size += self.write_index_range.byte_len();
        size += self.override_value_handling.byte_len();
        size += self.override_value.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.data_set_field_id.encode(stream)?;
        size += self.receiver_index_range.encode(stream)?;
        size += self.target_node_id.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.write_index_range.encode(stream)?;
        size += self.override_value_handling.encode(stream)?;
        size += self.override_value.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let data_set_field_id = <crate::types::guid::Guid as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let receiver_index_range = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let target_node_id = <crate::types::node_id::NodeId as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let attribute_id = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let write_index_range = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let override_value_handling = <super::enums::OverrideValueHandling as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let override_value = <crate::types::variant::Variant as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            data_set_field_id,
            receiver_index_range,
            target_node_id,
            attribute_id,
            write_index_range,
            override_value_handling,
            override_value,
        })
    }
}
