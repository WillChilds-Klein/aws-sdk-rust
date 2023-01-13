// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_start_zonal_shift_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartZonalShiftInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.away_from {
        object.key("awayFrom").string(var_1.as_str());
    }
    if let Some(var_2) = &input.comment {
        object.key("comment").string(var_2.as_str());
    }
    if let Some(var_3) = &input.expires_in {
        object.key("expiresIn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.resource_identifier {
        object.key("resourceIdentifier").string(var_4.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_zonal_shift_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateZonalShiftInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.comment {
        object.key("comment").string(var_5.as_str());
    }
    if let Some(var_6) = &input.expires_in {
        object.key("expiresIn").string(var_6.as_str());
    }
    Ok(())
}
