// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn expiration_settings_correct_errors(
    mut builder: crate::types::builders::ExpirationSettingsBuilder,
) -> crate::types::builders::ExpirationSettingsBuilder {
    if builder.expiration_days.is_none() {
        builder.expiration_days = Some(Default::default())
    }
    if builder.expiration_criterion.is_none() {
        builder.expiration_criterion = "no value was set".parse::<crate::types::ExpirationCriterion>().ok()
    }
    builder
}

pub(crate) fn configuration_correct_errors(
    mut builder: crate::types::builders::ConfigurationBuilder,
) -> crate::types::builders::ConfigurationBuilder {
    if builder.lex.is_none() {
        builder.lex = {
            let builder = crate::types::builders::LexConfigurationBuilder::default();
            crate::serde_util::lex_configuration_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn endpoint_attributes_correct_errors(
    mut builder: crate::types::builders::EndpointAttributesBuilder,
) -> crate::types::builders::EndpointAttributesBuilder {
    if builder.device_token.is_none() {
        builder.device_token = Some(Default::default())
    }
    builder
}

pub(crate) fn endpoint_state_correct_errors(
    mut builder: crate::types::builders::EndpointStateBuilder,
) -> crate::types::builders::EndpointStateBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::EndpointStatus>().ok()
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn lex_configuration_correct_errors(
    mut builder: crate::types::builders::LexConfigurationBuilder,
) -> crate::types::builders::LexConfigurationBuilder {
    if builder.lex_bot_alias_arn.is_none() {
        builder.lex_bot_alias_arn = Some(Default::default())
    }
    if builder.locale_id.is_none() {
        builder.locale_id = Some(Default::default())
    }
    builder
}

pub(crate) fn invoked_by_correct_errors(mut builder: crate::types::builders::InvokedByBuilder) -> crate::types::builders::InvokedByBuilder {
    if builder.standard_messages.is_none() {
        builder.standard_messages = "no value was set".parse::<crate::types::StandardMessages>().ok()
    }
    if builder.targeted_messages.is_none() {
        builder.targeted_messages = "no value was set".parse::<crate::types::TargetedMessages>().ok()
    }
    builder
}
