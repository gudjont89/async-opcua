// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct JsonWriterGroupMessageDataType {
    pub network_message_content_mask: super::enums::JsonNetworkMessageContentMask,
}
impl crate::types::BinaryEncoder for JsonWriterGroupMessageDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.network_message_content_mask.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.network_message_content_mask.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let network_message_content_mask = <super::enums::JsonNetworkMessageContentMask as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            network_message_content_mask,
        })
    }
}
