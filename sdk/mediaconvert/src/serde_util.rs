// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn job_correct_errors(mut builder: crate::types::builders::JobBuilder) -> crate::types::builders::JobBuilder {
    if builder.role.is_none() {
        builder.role = Some(Default::default())
    }
    if builder.settings.is_none() {
        builder.settings = {
            let builder = crate::types::builders::JobSettingsBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn job_template_correct_errors(mut builder: crate::types::builders::JobTemplateBuilder) -> crate::types::builders::JobTemplateBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.settings.is_none() {
        builder.settings = {
            let builder = crate::types::builders::JobTemplateSettingsBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn preset_correct_errors(mut builder: crate::types::builders::PresetBuilder) -> crate::types::builders::PresetBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.settings.is_none() {
        builder.settings = {
            let builder = crate::types::builders::PresetSettingsBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn queue_correct_errors(mut builder: crate::types::builders::QueueBuilder) -> crate::types::builders::QueueBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn acceleration_settings_correct_errors(
    mut builder: crate::types::builders::AccelerationSettingsBuilder,
) -> crate::types::builders::AccelerationSettingsBuilder {
    if builder.mode.is_none() {
        builder.mode = "no value was set".parse::<crate::types::AccelerationMode>().ok()
    }
    builder
}

pub(crate) fn warning_group_correct_errors(mut builder: crate::types::builders::WarningGroupBuilder) -> crate::types::builders::WarningGroupBuilder {
    if builder.code.is_none() {
        builder.code = Some(Default::default())
    }
    if builder.count.is_none() {
        builder.count = Some(Default::default())
    }
    builder
}
