// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attachment_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AttachmentInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        object.key("data").string_unchecked(&::aws_smithy_types::base64::encode(&input.data));
    }
    Ok(())
}
