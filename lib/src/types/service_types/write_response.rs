// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct WriteResponse {
    pub response_header: crate::types::response_header::ResponseHeader,
    pub results: Option<Vec<crate::types::status_code::StatusCode>>,
    pub diagnostic_infos: Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
}
impl crate::types::MessageInfo for WriteResponse {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::WriteResponse_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for WriteResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.results.byte_len();
        size += self.diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.results.encode(stream)?;
        size += self.diagnostic_infos.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let response_header = <crate::types::response_header::ResponseHeader as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = response_header.request_handle;
        let results = <Option<
            Vec<crate::types::status_code::StatusCode>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let diagnostic_infos = <Option<
            Vec<crate::types::diagnostic_info::DiagnosticInfo>,
        > as crate::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            response_header,
            results,
            diagnostic_infos,
        })
    }
}
