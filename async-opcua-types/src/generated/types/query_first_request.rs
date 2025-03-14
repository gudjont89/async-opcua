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
pub struct QueryFirstRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub view: super::view_description::ViewDescription,
    pub node_types: Option<Vec<super::node_type_description::NodeTypeDescription>>,
    pub filter: super::content_filter::ContentFilter,
    pub max_data_sets_to_return: u32,
    pub max_references_to_return: u32,
}
impl opcua::types::MessageInfo for QueryFirstRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QueryFirstRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QueryFirstRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::QueryFirstRequest_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::QueryFirstRequest
    }
}
