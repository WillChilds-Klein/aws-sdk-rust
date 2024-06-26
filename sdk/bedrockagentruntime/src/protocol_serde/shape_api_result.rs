// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_api_result(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ApiResult,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("actionGroup").string(input.action_group.as_str());
    }
    if let Some(var_1) = &input.http_method {
        object.key("httpMethod").string(var_1.as_str());
    }
    if let Some(var_2) = &input.api_path {
        object.key("apiPath").string(var_2.as_str());
    }
    if let Some(var_3) = &input.response_body {
        #[allow(unused_mut)]
        let mut object_4 = object.key("responseBody").start_object();
        for (key_5, value_6) in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_7 = object_4.key(key_5.as_str()).start_object();
                crate::protocol_serde::shape_content_body::ser_content_body(&mut object_7, value_6)?;
                object_7.finish();
            }
        }
        object_4.finish();
    }
    if let Some(var_8) = &input.http_status_code {
        object.key("httpStatusCode").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.response_state {
        object.key("responseState").string(var_9.as_str());
    }
    Ok(())
}
