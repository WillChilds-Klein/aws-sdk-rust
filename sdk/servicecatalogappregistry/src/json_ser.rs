// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("tags").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_attribute_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAttributeGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.attributes {
        object.key("attributes").string(var_8.as_str());
    }
    if let Some(var_9) = &input.client_token {
        object.key("clientToken").string(var_9.as_str());
    }
    if let Some(var_10) = &input.description {
        object.key("description").string(var_10.as_str());
    }
    if let Some(var_11) = &input.name {
        object.key("name").string(var_11.as_str());
    }
    if let Some(var_12) = &input.tags {
        #[allow(unused_mut)]
        let mut object_13 = object.key("tags").start_object();
        for (key_14, value_15) in var_12 {
            {
                object_13.key(key_14.as_str()).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_17 = object.key("configuration").start_object();
        crate::json_ser::serialize_structure_crate_model_app_registry_configuration(
            &mut object_17,
            var_16,
        )?;
        object_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.tags {
        #[allow(unused_mut)]
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateApplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.description {
        object.key("description").string(var_22.as_str());
    }
    if let Some(var_23) = &input.name {
        object.key("name").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_attribute_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAttributeGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.attributes {
        object.key("attributes").string(var_24.as_str());
    }
    if let Some(var_25) = &input.description {
        object.key("description").string(var_25.as_str());
    }
    if let Some(var_26) = &input.name {
        object.key("name").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_app_registry_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AppRegistryConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.tag_query_configuration {
        #[allow(unused_mut)]
        let mut object_28 = object.key("tagQueryConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_tag_query_configuration(
            &mut object_28,
            var_27,
        )?;
        object_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag_query_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TagQueryConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_29) = &input.tag_key {
        object.key("tagKey").string(var_29.as_str());
    }
    Ok(())
}
