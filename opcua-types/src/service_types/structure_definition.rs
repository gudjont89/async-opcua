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
pub struct StructureDefinition {
    pub default_encoding_id: opcua::types::node_id::NodeId,
    pub base_data_type: opcua::types::node_id::NodeId,
    pub structure_type: super::enums::StructureType,
    pub fields: Option<Vec<super::structure_field::StructureField>>,
}
impl opcua::types::BinaryEncodable for StructureDefinition {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.default_encoding_id.byte_len();
        size += self.base_data_type.byte_len();
        size += self.structure_type.byte_len();
        size += self.fields.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.default_encoding_id.encode(stream)?;
        size += self.base_data_type.encode(stream)?;
        size += self.structure_type.encode(stream)?;
        size += self.fields.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for StructureDefinition {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            default_encoding_id: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            base_data_type: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            structure_type: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            fields: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
