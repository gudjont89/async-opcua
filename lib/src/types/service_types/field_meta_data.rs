// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct FieldMetaData {
    pub name: crate::types::string::UAString,
    pub description: crate::types::localized_text::LocalizedText,
    pub field_flags: super::enums::DataSetFieldFlags,
    pub built_in_type: u8,
    pub data_type: crate::types::node_id::NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub max_string_length: u32,
    pub data_set_field_id: crate::types::guid::Guid,
    pub properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
}
impl crate::types::MessageInfo for FieldMetaData {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::FieldMetaData_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for FieldMetaData {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.field_flags.byte_len();
        size += self.built_in_type.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += self.array_dimensions.byte_len();
        size += self.max_string_length.byte_len();
        size += self.data_set_field_id.byte_len();
        size += self.properties.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.field_flags.encode(stream)?;
        size += self.built_in_type.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += self.array_dimensions.encode(stream)?;
        size += self.max_string_length.encode(stream)?;
        size += self.data_set_field_id.encode(stream)?;
        size += self.properties.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let name = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let description = <crate::types::localized_text::LocalizedText as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let field_flags = <super::enums::DataSetFieldFlags as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let built_in_type = <u8 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_type = <crate::types::node_id::NodeId as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let value_rank = <i32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let array_dimensions = <Option<
            Vec<u32>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let max_string_length = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_field_id = <crate::types::guid::Guid as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            name,
            description,
            field_flags,
            built_in_type,
            data_type,
            value_rank,
            array_dimensions,
            max_string_length,
            data_set_field_id,
            properties,
        })
    }
}
