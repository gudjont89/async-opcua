// This file was autogenerated from schemas/1.05/Opc.Ua.Types.bsd by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct BuildInfo {
    pub product_uri: opcua::types::string::UAString,
    pub manufacturer_name: opcua::types::string::UAString,
    pub product_name: opcua::types::string::UAString,
    pub software_version: opcua::types::string::UAString,
    pub build_number: opcua::types::string::UAString,
    pub build_date: opcua::types::date_time::DateTime,
}
impl opcua::types::MessageInfo for BuildInfo {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::BuildInfo
    }
}
