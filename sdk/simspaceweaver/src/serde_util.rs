// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn s3_location_correct_errors(mut builder: crate::types::builders::S3LocationBuilder) -> crate::types::builders::S3LocationBuilder {
    if builder.bucket_name.is_none() {
        builder.bucket_name = Some(Default::default())
    }
    if builder.object_key.is_none() {
        builder.object_key = Some(Default::default())
    }
    builder
}
