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
pub struct Argument {
    pub name: opcua::types::string::UAString,
    pub data_type: opcua::types::node_id::NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub description: opcua::types::localized_text::LocalizedText,
}
impl opcua::types::MessageInfo for Argument {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::Argument_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::Argument_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::Argument_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for Argument {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += self.array_dimensions.byte_len();
        size += self.description.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write + ?Sized>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += self.array_dimensions.encode(stream)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }
}
impl opcua::types::BinaryDecodable for Argument {
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            name: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            data_type: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            value_rank: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
            array_dimensions: opcua::types::BinaryDecodable::decode(
                stream,
                decoding_options,
            )?,
            description: opcua::types::BinaryDecodable::decode(stream, decoding_options)?,
        })
    }
}
