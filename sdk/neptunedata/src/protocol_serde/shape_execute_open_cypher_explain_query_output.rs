// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_results_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<::aws_smithy_types::Blob>,
    crate::operation::execute_open_cypher_explain_query::ExecuteOpenCypherExplainQueryError,
> {
    (!body.is_empty()).then(|| Ok(::aws_smithy_types::Blob::new(body))).transpose()
}
