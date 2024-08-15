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
pub struct ServerDiagnosticsSummaryDataType {
    pub server_view_count: u32,
    pub current_session_count: u32,
    pub cumulated_session_count: u32,
    pub security_rejected_session_count: u32,
    pub rejected_session_count: u32,
    pub session_timeout_count: u32,
    pub session_abort_count: u32,
    pub current_subscription_count: u32,
    pub cumulated_subscription_count: u32,
    pub publishing_interval_count: u32,
    pub security_rejected_requests_count: u32,
    pub rejected_requests_count: u32,
}
impl crate::types::MessageInfo for ServerDiagnosticsSummaryDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::ServerDiagnosticsSummaryDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder for ServerDiagnosticsSummaryDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.server_view_count.byte_len();
        size += self.current_session_count.byte_len();
        size += self.cumulated_session_count.byte_len();
        size += self.security_rejected_session_count.byte_len();
        size += self.rejected_session_count.byte_len();
        size += self.session_timeout_count.byte_len();
        size += self.session_abort_count.byte_len();
        size += self.current_subscription_count.byte_len();
        size += self.cumulated_subscription_count.byte_len();
        size += self.publishing_interval_count.byte_len();
        size += self.security_rejected_requests_count.byte_len();
        size += self.rejected_requests_count.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.server_view_count.encode(stream)?;
        size += self.current_session_count.encode(stream)?;
        size += self.cumulated_session_count.encode(stream)?;
        size += self.security_rejected_session_count.encode(stream)?;
        size += self.rejected_session_count.encode(stream)?;
        size += self.session_timeout_count.encode(stream)?;
        size += self.session_abort_count.encode(stream)?;
        size += self.current_subscription_count.encode(stream)?;
        size += self.cumulated_subscription_count.encode(stream)?;
        size += self.publishing_interval_count.encode(stream)?;
        size += self.security_rejected_requests_count.encode(stream)?;
        size += self.rejected_requests_count.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let server_view_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let current_session_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let cumulated_session_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_rejected_session_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let rejected_session_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let session_timeout_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let session_abort_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let current_subscription_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let cumulated_subscription_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let publishing_interval_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let security_rejected_requests_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let rejected_requests_count = <u32 as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            server_view_count,
            current_session_count,
            cumulated_session_count,
            security_rejected_session_count,
            rejected_session_count,
            session_timeout_count,
            session_abort_count,
            current_subscription_count,
            cumulated_subscription_count,
            publishing_interval_count,
            security_rejected_requests_count,
            rejected_requests_count,
        })
    }
}
