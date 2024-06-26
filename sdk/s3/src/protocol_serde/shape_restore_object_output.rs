// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_request_charged_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<crate::types::RequestCharged>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-request-charged");
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_restore_output_path_header(
    header_map: &::aws_smithy_runtime_api::http::Headers,
) -> ::std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-restore-output-path");
    ::aws_smithy_http::header::one_or_none(headers)
}
