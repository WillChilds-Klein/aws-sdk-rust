// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_connection_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ConnectionInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Name").string(input.name.as_str());
    }
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    {
        object.key("ConnectionType").string(input.connection_type.as_str());
    }
    if let Some(var_2) = &input.match_criteria {
        let mut array_3 = object.key("MatchCriteria").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ConnectionProperties").start_object();
        for (key_6, value_7) in &input.connection_properties {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.physical_connection_requirements {
        #[allow(unused_mut)]
        let mut object_9 = object.key("PhysicalConnectionRequirements").start_object();
        crate::protocol_serde::shape_physical_connection_requirements::ser_physical_connection_requirements(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}
