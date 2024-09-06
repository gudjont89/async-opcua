// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct DataSetMetaDataType {
    pub namespaces: Option<Vec<opcua::types::string::UAString>>,
    pub structure_data_types: Option<
        Vec<super::structure_description::StructureDescription>,
    >,
    pub enum_data_types: Option<Vec<super::enum_description::EnumDescription>>,
    pub simple_data_types: Option<
        Vec<super::simple_type_description::SimpleTypeDescription>,
    >,
    pub name: opcua::types::string::UAString,
    pub description: opcua::types::localized_text::LocalizedText,
    pub fields: Option<Vec<super::field_meta_data::FieldMetaData>>,
    pub data_set_class_id: opcua::types::guid::Guid,
    pub configuration_version: super::configuration_version_data_type::ConfigurationVersionDataType,
}
impl opcua::types::MessageInfo for DataSetMetaDataType {
    fn object_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::DataSetMetaDataType_Encoding_DefaultBinary
    }
}
impl opcua::types::BinaryEncoder for DataSetMetaDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.namespaces.byte_len();
        size += self.structure_data_types.byte_len();
        size += self.enum_data_types.byte_len();
        size += self.simple_data_types.byte_len();
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.fields.byte_len();
        size += self.data_set_class_id.byte_len();
        size += self.configuration_version.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.namespaces.encode(stream)?;
        size += self.structure_data_types.encode(stream)?;
        size += self.enum_data_types.encode(stream)?;
        size += self.simple_data_types.encode(stream)?;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.fields.encode(stream)?;
        size += self.data_set_class_id.encode(stream)?;
        size += self.configuration_version.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let namespaces = <Option<
            Vec<opcua::types::string::UAString>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let structure_data_types = <Option<
            Vec<super::structure_description::StructureDescription>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let enum_data_types = <Option<
            Vec<super::enum_description::EnumDescription>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let simple_data_types = <Option<
            Vec<super::simple_type_description::SimpleTypeDescription>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let name = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let description = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let fields = <Option<
            Vec<super::field_meta_data::FieldMetaData>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let data_set_class_id = <opcua::types::guid::Guid as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let configuration_version = <super::configuration_version_data_type::ConfigurationVersionDataType as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            namespaces,
            structure_data_types,
            enum_data_types,
            simple_data_types,
            name,
            description,
            fields,
            data_set_class_id,
            configuration_version,
        })
    }
}
