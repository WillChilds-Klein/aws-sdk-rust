// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCisScanConfiguration`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`scan_configuration_arn(impl Into<String>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::scan_configuration_arn) / [`set_scan_configuration_arn(Option<String>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::set_scan_configuration_arn):<br>required: **true**<br><p>The CIS scan configuration ARN.</p><br>
    ///   - [`scan_name(impl Into<String>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::scan_name) / [`set_scan_name(Option<String>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::set_scan_name):<br>required: **false**<br><p>The scan name for the CIS scan configuration.</p><br>
    ///   - [`security_level(CisSecurityLevel)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::security_level) / [`set_security_level(Option<CisSecurityLevel>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::set_security_level):<br>required: **false**<br><p>The security level for the CIS scan configuration. Security level refers to the Benchmark levels that CIS assigns to a profile.</p><br>
    ///   - [`schedule(Schedule)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::schedule) / [`set_schedule(Option<Schedule>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::set_schedule):<br>required: **false**<br><p>The schedule for the CIS scan configuration.</p><br>
    ///   - [`targets(UpdateCisTargets)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::targets) / [`set_targets(Option<UpdateCisTargets>)`](crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::set_targets):<br>required: **false**<br><p>The targets for the CIS scan configuration.</p><br>
    /// - On success, responds with [`UpdateCisScanConfigurationOutput`](crate::operation::update_cis_scan_configuration::UpdateCisScanConfigurationOutput) with field(s):
    ///   - [`scan_configuration_arn(String)`](crate::operation::update_cis_scan_configuration::UpdateCisScanConfigurationOutput::scan_configuration_arn): <p>The CIS scan configuration ARN.</p>
    /// - On failure, responds with [`SdkError<UpdateCisScanConfigurationError>`](crate::operation::update_cis_scan_configuration::UpdateCisScanConfigurationError)
    pub fn update_cis_scan_configuration(
        &self,
    ) -> crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder {
        crate::operation::update_cis_scan_configuration::builders::UpdateCisScanConfigurationFluentBuilder::new(self.handle.clone())
    }
}
