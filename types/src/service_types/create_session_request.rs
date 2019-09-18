// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
    string::UAString,
    byte_string::ByteString,
    service_types::ApplicationDescription,
};

/// Creates a new session with the server.
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSessionRequest {
    pub request_header: RequestHeader,
    pub client_description: ApplicationDescription,
    pub server_uri: UAString,
    pub endpoint_url: UAString,
    pub session_name: UAString,
    pub client_nonce: ByteString,
    pub client_certificate: ByteString,
    pub requested_session_timeout: f64,
    pub max_response_message_size: u32,
}

impl MessageInfo for CreateSessionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateSessionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateSessionRequest> for CreateSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_description.byte_len();
        size += self.server_uri.byte_len();
        size += self.endpoint_url.byte_len();
        size += self.session_name.byte_len();
        size += self.client_nonce.byte_len();
        size += self.client_certificate.byte_len();
        size += self.requested_session_timeout.byte_len();
        size += self.max_response_message_size.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_description.encode(stream)?;
        size += self.server_uri.encode(stream)?;
        size += self.endpoint_url.encode(stream)?;
        size += self.session_name.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.client_certificate.encode(stream)?;
        size += self.requested_session_timeout.encode(stream)?;
        size += self.max_response_message_size.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let client_description = ApplicationDescription::decode(stream, decoding_limits)?;
        let server_uri = UAString::decode(stream, decoding_limits)?;
        let endpoint_url = UAString::decode(stream, decoding_limits)?;
        let session_name = UAString::decode(stream, decoding_limits)?;
        let client_nonce = ByteString::decode(stream, decoding_limits)?;
        let client_certificate = ByteString::decode(stream, decoding_limits)?;
        let requested_session_timeout = f64::decode(stream, decoding_limits)?;
        let max_response_message_size = u32::decode(stream, decoding_limits)?;
        Ok(CreateSessionRequest {
            request_header,
            client_description,
            server_uri,
            endpoint_url,
            session_name,
            client_nonce,
            client_certificate,
            requested_session_timeout,
            max_response_message_size,
        })
    }
}
