// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq)]
pub struct BrokerDataSetReaderTransportDataType {
    pub queue_name: crate::types::string::UAString,
    pub resource_uri: crate::types::string::UAString,
    pub authentication_profile_uri: crate::types::string::UAString,
    pub requested_delivery_guarantee: super::enums::BrokerTransportQualityOfService,
    pub meta_data_queue_name: crate::types::string::UAString,
}
impl crate::types::BinaryEncoder for BrokerDataSetReaderTransportDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.queue_name.byte_len();
        size += self.resource_uri.byte_len();
        size += self.authentication_profile_uri.byte_len();
        size += self.requested_delivery_guarantee.byte_len();
        size += self.meta_data_queue_name.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.queue_name.encode(stream)?;
        size += self.resource_uri.encode(stream)?;
        size += self.authentication_profile_uri.encode(stream)?;
        size += self.requested_delivery_guarantee.encode(stream)?;
        size += self.meta_data_queue_name.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let queue_name = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let resource_uri = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let authentication_profile_uri = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let requested_delivery_guarantee = <super::enums::BrokerTransportQualityOfService as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let meta_data_queue_name = <crate::types::string::UAString as crate::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            queue_name,
            resource_uri,
            authentication_profile_uri,
            requested_delivery_guarantee,
            meta_data_queue_name,
        })
    }
}
