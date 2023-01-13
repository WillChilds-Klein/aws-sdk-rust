// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_slack_channel_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSlackChannelConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.channel_id {
        object.key("channelId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.channel_name {
        object.key("channelName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.channel_role_arn {
        object.key("channelRoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.notify_on_add_correspondence_to_case {
        object
            .key("notifyOnAddCorrespondenceToCase")
            .boolean(*var_4);
    }
    if let Some(var_5) = &input.notify_on_case_severity {
        object.key("notifyOnCaseSeverity").string(var_5.as_str());
    }
    if let Some(var_6) = &input.notify_on_create_or_reopen_case {
        object.key("notifyOnCreateOrReopenCase").boolean(*var_6);
    }
    if let Some(var_7) = &input.notify_on_resolve_case {
        object.key("notifyOnResolveCase").boolean(*var_7);
    }
    if let Some(var_8) = &input.team_id {
        object.key("teamId").string(var_8.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_slack_channel_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSlackChannelConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.channel_id {
        object.key("channelId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.team_id {
        object.key("teamId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_slack_workspace_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSlackWorkspaceConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.team_id {
        object.key("teamId").string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_slack_channel_configurations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSlackChannelConfigurationsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.next_token {
        object.key("nextToken").string(var_12.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_slack_workspace_configurations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSlackWorkspaceConfigurationsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_13) = &input.next_token {
        object.key("nextToken").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_account_alias_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAccountAliasInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.account_alias {
        object.key("accountAlias").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_slack_workspace_for_organization_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterSlackWorkspaceForOrganizationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.team_id {
        object.key("teamId").string(var_15.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_slack_channel_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSlackChannelConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.channel_id {
        object.key("channelId").string(var_16.as_str());
    }
    if let Some(var_17) = &input.channel_name {
        object.key("channelName").string(var_17.as_str());
    }
    if let Some(var_18) = &input.channel_role_arn {
        object.key("channelRoleArn").string(var_18.as_str());
    }
    if let Some(var_19) = &input.notify_on_add_correspondence_to_case {
        object
            .key("notifyOnAddCorrespondenceToCase")
            .boolean(*var_19);
    }
    if let Some(var_20) = &input.notify_on_case_severity {
        object.key("notifyOnCaseSeverity").string(var_20.as_str());
    }
    if let Some(var_21) = &input.notify_on_create_or_reopen_case {
        object.key("notifyOnCreateOrReopenCase").boolean(*var_21);
    }
    if let Some(var_22) = &input.notify_on_resolve_case {
        object.key("notifyOnResolveCase").boolean(*var_22);
    }
    if let Some(var_23) = &input.team_id {
        object.key("teamId").string(var_23.as_str());
    }
    Ok(())
}
