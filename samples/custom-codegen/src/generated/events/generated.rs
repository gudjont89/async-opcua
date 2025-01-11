// This file was autogenerated from schema/Opc.Ua.Pn.NodeSet2.xml by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Einar Omang
#[allow(unused)]
mod types {
    pub use crate::generated::types::*;
    pub use ::opcua::types::*;
}
#[derive(Debug, opcua::Event)]
#[opcua(
    identifier = "i=1003",
    namespace = "http://opcfoundation.org/UA/PROFINET/"
)]
pub struct PnAssetChangedEventType {
    pub base: opcua::nodes::BaseEventType,
    pub own_namespace_index: u16,
    pub asset_change: types::PnAssetChangeEnumeration,
    pub asset_type: types::PnAssetTypeEnumeration,
}
#[derive(Debug, opcua::Event)]
#[opcua(
    identifier = "i=1002",
    namespace = "http://opcfoundation.org/UA/PROFINET/"
)]
pub struct PnDiagnosisAlarmType {
    pub base: opcua::core_namespace::events::AlarmConditionType,
    pub own_namespace_index: u16,
    pub api: u32,
    pub accumulative: types::PnChannelAccumulativeEnumeration,
    pub channel_error_type: u16,
    pub channel_number: u16,
    pub direction: types::PnChannelDirectionEnumeration,
    pub ext_channel_add_value: u32,
    pub ext_channel_error_type: u16,
    pub help_text: types::LocalizedText,
    pub maintenance: types::PnChannelMaintenanceEnumeration,
    pub manufacturer_data: types::ByteString,
    pub qualified_channel_qualifier: u32,
    pub slot: u16,
    pub specifier: types::PnChannelSpecifierEnumeration,
    pub subslot: u16,
    #[opcua(rename = "Type")]
    pub __type: types::PnChannelTypeEnumeration,
    pub user_structure_identifier: u16,
}
#[derive(Debug, opcua::Event)]
#[opcua(
    identifier = "i=1004",
    namespace = "http://opcfoundation.org/UA/PROFINET/"
)]
pub struct PnTopologyChangedEventType {
    pub base: opcua::nodes::BaseEventType,
    pub own_namespace_index: u16,
}
