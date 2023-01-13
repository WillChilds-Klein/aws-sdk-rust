// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_get_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchGetRecordInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identifiers {
        let mut array_2 = object.key("Identifiers").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_batch_get_record_identifier(
                    &mut object_4,
                    item_3,
                )?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_record_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutRecordInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.record {
        let mut array_6 = object.key("Record").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_feature_value(
                    &mut object_8,
                    item_7,
                )?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.target_stores {
        let mut array_10 = object.key("TargetStores").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_batch_get_record_identifier(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::BatchGetRecordIdentifier,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.feature_group_name {
        object.key("FeatureGroupName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.record_identifiers_value_as_string {
        let mut array_14 = object.key("RecordIdentifiersValueAsString").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.feature_names {
        let mut array_17 = object.key("FeatureNames").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_feature_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FeatureValue,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.feature_name {
        object.key("FeatureName").string(var_19.as_str());
    }
    if let Some(var_20) = &input.value_as_string {
        object.key("ValueAsString").string(var_20.as_str());
    }
    Ok(())
}
