// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct VariableAttributes {
    pub specified_attributes: u32,
    pub display_name: opcua::types::localized_text::LocalizedText,
    pub description: opcua::types::localized_text::LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub value: opcua::types::variant::Variant,
    pub data_type: opcua::types::node_id::NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub access_level: u8,
    pub user_access_level: u8,
    pub minimum_sampling_interval: f64,
    pub historizing: bool,
}
impl opcua::types::MessageInfo for VariableAttributes {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::VariableAttributes_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::VariableAttributes_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::VariableAttributes_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for VariableAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += self.value.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += self.array_dimensions.byte_len();
        size += self.access_level.byte_len();
        size += self.user_access_level.byte_len();
        size += self.minimum_sampling_interval.byte_len();
        size += self.historizing.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += self.value.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += self.array_dimensions.encode(stream)?;
        size += self.access_level.encode(stream)?;
        size += self.user_access_level.encode(stream)?;
        size += self.minimum_sampling_interval.encode(stream)?;
        size += self.historizing.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for VariableAttributes {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            specified_attributes: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            display_name: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            description: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            write_mask: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            user_write_mask: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            value: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            data_type: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            value_rank: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            array_dimensions: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            access_level: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            user_access_level: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            minimum_sampling_interval: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            historizing: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
