// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_event_stream_payload(
    body: &mut ::aws_smithy_types::body::SdkBody,
) -> std::result::Result<
    crate::event_receiver::EventReceiver<
        crate::types::InvokeWithResponseStreamResponseEvent,
        crate::types::error::InvokeWithResponseStreamResponseEventError,
    >,
    crate::operation::invoke_with_response_stream::InvokeWithResponseStreamError,
> {
    let unmarshaller = crate::event_stream_serde::InvokeWithResponseStreamResponseEventUnmarshaller::new();
    let body = std::mem::replace(body, ::aws_smithy_types::body::SdkBody::taken());
    Ok(crate::event_receiver::EventReceiver::new(::aws_smithy_http::event_stream::Receiver::new(
        unmarshaller,
        body,
    )))
}

pub(crate) fn de_executed_version_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("X-Amz-Executed-Version");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_response_stream_content_type_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Content-Type");
    ::aws_smithy_http::header::one_or_none(headers)
}
