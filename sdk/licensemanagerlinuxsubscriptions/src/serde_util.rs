// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn linux_subscriptions_discovery_settings_correct_errors(
    mut builder: crate::types::builders::LinuxSubscriptionsDiscoverySettingsBuilder,
) -> crate::types::builders::LinuxSubscriptionsDiscoverySettingsBuilder {
    if builder.source_regions.is_none() {
        builder.source_regions = Some(Default::default())
    }
    if builder.organization_integration.is_none() {
        builder.organization_integration = "no value was set".parse::<crate::types::OrganizationIntegration>().ok()
    }
    builder
}
