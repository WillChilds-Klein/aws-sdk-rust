// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_schedule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateScheduleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.end_date {
        object
            .key("EndDate")
            .date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.flexible_time_window {
        #[allow(unused_mut)]
        let mut object_5 = object.key("FlexibleTimeWindow").start_object();
        crate::json_ser::serialize_structure_crate_model_flexible_time_window(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    if let Some(var_6) = &input.group_name {
        object.key("GroupName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_7.as_str());
    }
    if let Some(var_8) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_8.as_str());
    }
    if let Some(var_9) = &input.schedule_expression_timezone {
        object
            .key("ScheduleExpressionTimezone")
            .string(var_9.as_str());
    }
    if let Some(var_10) = &input.start_date {
        object
            .key("StartDate")
            .date_time(var_10, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_11) = &input.state {
        object.key("State").string(var_11.as_str());
    }
    if let Some(var_12) = &input.target {
        #[allow(unused_mut)]
        let mut object_13 = object.key("Target").start_object();
        crate::json_ser::serialize_structure_crate_model_target(&mut object_13, var_12)?;
        object_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_schedule_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateScheduleGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.client_token {
        object.key("ClientToken").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut array_16 = object.key("Tags").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.tags {
        let mut array_20 = object.key("Tags").start_array();
        for item_21 in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_schedule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateScheduleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.client_token {
        object.key("ClientToken").string(var_23.as_str());
    }
    if let Some(var_24) = &input.description {
        object.key("Description").string(var_24.as_str());
    }
    if let Some(var_25) = &input.end_date {
        object
            .key("EndDate")
            .date_time(var_25, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_26) = &input.flexible_time_window {
        #[allow(unused_mut)]
        let mut object_27 = object.key("FlexibleTimeWindow").start_object();
        crate::json_ser::serialize_structure_crate_model_flexible_time_window(
            &mut object_27,
            var_26,
        )?;
        object_27.finish();
    }
    if let Some(var_28) = &input.group_name {
        object.key("GroupName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_29.as_str());
    }
    if let Some(var_30) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_30.as_str());
    }
    if let Some(var_31) = &input.schedule_expression_timezone {
        object
            .key("ScheduleExpressionTimezone")
            .string(var_31.as_str());
    }
    if let Some(var_32) = &input.start_date {
        object
            .key("StartDate")
            .date_time(var_32, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_33) = &input.state {
        object.key("State").string(var_33.as_str());
    }
    if let Some(var_34) = &input.target {
        #[allow(unused_mut)]
        let mut object_35 = object.key("Target").start_object();
        crate::json_ser::serialize_structure_crate_model_target(&mut object_35, var_34)?;
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_flexible_time_window(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FlexibleTimeWindow,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.mode {
        object.key("Mode").string(var_36.as_str());
    }
    if let Some(var_37) = &input.maximum_window_in_minutes {
        object.key("MaximumWindowInMinutes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_target(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Target,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.arn {
        object.key("Arn").string(var_38.as_str());
    }
    if let Some(var_39) = &input.role_arn {
        object.key("RoleArn").string(var_39.as_str());
    }
    if let Some(var_40) = &input.dead_letter_config {
        #[allow(unused_mut)]
        let mut object_41 = object.key("DeadLetterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dead_letter_config(
            &mut object_41,
            var_40,
        )?;
        object_41.finish();
    }
    if let Some(var_42) = &input.retry_policy {
        #[allow(unused_mut)]
        let mut object_43 = object.key("RetryPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_retry_policy(&mut object_43, var_42)?;
        object_43.finish();
    }
    if let Some(var_44) = &input.input {
        object.key("Input").string(var_44.as_str());
    }
    if let Some(var_45) = &input.ecs_parameters {
        #[allow(unused_mut)]
        let mut object_46 = object.key("EcsParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_ecs_parameters(&mut object_46, var_45)?;
        object_46.finish();
    }
    if let Some(var_47) = &input.event_bridge_parameters {
        #[allow(unused_mut)]
        let mut object_48 = object.key("EventBridgeParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_event_bridge_parameters(
            &mut object_48,
            var_47,
        )?;
        object_48.finish();
    }
    if let Some(var_49) = &input.kinesis_parameters {
        #[allow(unused_mut)]
        let mut object_50 = object.key("KinesisParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_kinesis_parameters(
            &mut object_50,
            var_49,
        )?;
        object_50.finish();
    }
    if let Some(var_51) = &input.sage_maker_pipeline_parameters {
        #[allow(unused_mut)]
        let mut object_52 = object.key("SageMakerPipelineParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_sage_maker_pipeline_parameters(
            &mut object_52,
            var_51,
        )?;
        object_52.finish();
    }
    if let Some(var_53) = &input.sqs_parameters {
        #[allow(unused_mut)]
        let mut object_54 = object.key("SqsParameters").start_object();
        crate::json_ser::serialize_structure_crate_model_sqs_parameters(&mut object_54, var_53)?;
        object_54.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_55) = &input.key {
        object.key("Key").string(var_55.as_str());
    }
    if let Some(var_56) = &input.value {
        object.key("Value").string(var_56.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dead_letter_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeadLetterConfig,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.arn {
        object.key("Arn").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_retry_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RetryPolicy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.maximum_event_age_in_seconds {
        object.key("MaximumEventAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_58).into()),
        );
    }
    if let Some(var_59) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_59).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ecs_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EcsParameters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.task_definition_arn {
        object.key("TaskDefinitionArn").string(var_60.as_str());
    }
    if let Some(var_61) = &input.task_count {
        object.key("TaskCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.launch_type {
        object.key("LaunchType").string(var_62.as_str());
    }
    if let Some(var_63) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_64 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_configuration(
            &mut object_64,
            var_63,
        )?;
        object_64.finish();
    }
    if let Some(var_65) = &input.platform_version {
        object.key("PlatformVersion").string(var_65.as_str());
    }
    if let Some(var_66) = &input.group {
        object.key("Group").string(var_66.as_str());
    }
    if let Some(var_67) = &input.capacity_provider_strategy {
        let mut array_68 = object.key("CapacityProviderStrategy").start_array();
        for item_69 in var_67 {
            {
                #[allow(unused_mut)]
                let mut object_70 = array_68.value().start_object();
                crate::json_ser::serialize_structure_crate_model_capacity_provider_strategy_item(
                    &mut object_70,
                    item_69,
                )?;
                object_70.finish();
            }
        }
        array_68.finish();
    }
    if let Some(var_71) = &input.enable_ecs_managed_tags {
        object.key("EnableECSManagedTags").boolean(*var_71);
    }
    if let Some(var_72) = &input.enable_execute_command {
        object.key("EnableExecuteCommand").boolean(*var_72);
    }
    if let Some(var_73) = &input.placement_constraints {
        let mut array_74 = object.key("PlacementConstraints").start_array();
        for item_75 in var_73 {
            {
                #[allow(unused_mut)]
                let mut object_76 = array_74.value().start_object();
                crate::json_ser::serialize_structure_crate_model_placement_constraint(
                    &mut object_76,
                    item_75,
                )?;
                object_76.finish();
            }
        }
        array_74.finish();
    }
    if let Some(var_77) = &input.placement_strategy {
        let mut array_78 = object.key("PlacementStrategy").start_array();
        for item_79 in var_77 {
            {
                #[allow(unused_mut)]
                let mut object_80 = array_78.value().start_object();
                crate::json_ser::serialize_structure_crate_model_placement_strategy(
                    &mut object_80,
                    item_79,
                )?;
                object_80.finish();
            }
        }
        array_78.finish();
    }
    if let Some(var_81) = &input.propagate_tags {
        object.key("PropagateTags").string(var_81.as_str());
    }
    if let Some(var_82) = &input.reference_id {
        object.key("ReferenceId").string(var_82.as_str());
    }
    if let Some(var_83) = &input.tags {
        let mut array_84 = object.key("Tags").start_array();
        for item_85 in var_83 {
            {
                #[allow(unused_mut)]
                let mut object_86 = array_84.value().start_object();
                for (key_87, value_88) in item_85 {
                    {
                        object_86.key(key_87.as_str()).string(value_88.as_str());
                    }
                }
                object_86.finish();
            }
        }
        array_84.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_bridge_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventBridgeParameters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.detail_type {
        object.key("DetailType").string(var_89.as_str());
    }
    if let Some(var_90) = &input.source {
        object.key("Source").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_kinesis_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KinesisParameters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.partition_key {
        object.key("PartitionKey").string(var_91.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sage_maker_pipeline_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SageMakerPipelineParameters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.pipeline_parameter_list {
        let mut array_93 = object.key("PipelineParameterList").start_array();
        for item_94 in var_92 {
            {
                #[allow(unused_mut)]
                let mut object_95 = array_93.value().start_object();
                crate::json_ser::serialize_structure_crate_model_sage_maker_pipeline_parameter(
                    &mut object_95,
                    item_94,
                )?;
                object_95.finish();
            }
        }
        array_93.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sqs_parameters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SqsParameters,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.message_group_id {
        object.key("MessageGroupId").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NetworkConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_97) = &input.awsvpc_configuration {
        #[allow(unused_mut)]
        let mut object_98 = object.key("awsvpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_aws_vpc_configuration(
            &mut object_98,
            var_97,
        )?;
        object_98.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_capacity_provider_strategy_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CapacityProviderStrategyItem,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.capacity_provider {
        object.key("capacityProvider").string(var_99.as_str());
    }
    if input.weight != 0 {
        object.key("weight").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.weight).into()),
        );
    }
    if input.base != 0 {
        object.key("base").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.base).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_placement_constraint(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PlacementConstraint,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.r#type {
        object.key("type").string(var_100.as_str());
    }
    if let Some(var_101) = &input.expression {
        object.key("expression").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_placement_strategy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PlacementStrategy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_102) = &input.r#type {
        object.key("type").string(var_102.as_str());
    }
    if let Some(var_103) = &input.field {
        object.key("field").string(var_103.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sage_maker_pipeline_parameter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SageMakerPipelineParameter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_104) = &input.name {
        object.key("Name").string(var_104.as_str());
    }
    if let Some(var_105) = &input.value {
        object.key("Value").string(var_105.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_aws_vpc_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AwsVpcConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_106) = &input.subnets {
        let mut array_107 = object.key("Subnets").start_array();
        for item_108 in var_106 {
            {
                array_107.value().string(item_108.as_str());
            }
        }
        array_107.finish();
    }
    if let Some(var_109) = &input.security_groups {
        let mut array_110 = object.key("SecurityGroups").start_array();
        for item_111 in var_109 {
            {
                array_110.value().string(item_111.as_str());
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.assign_public_ip {
        object.key("AssignPublicIp").string(var_112.as_str());
    }
    Ok(())
}
