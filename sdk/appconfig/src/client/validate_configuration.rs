// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ValidateConfiguration`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::set_application_id):<br>required: **true**<br><p>The application ID.</p><br>
    ///   - [`configuration_profile_id(impl Into<String>)`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::configuration_profile_id) / [`set_configuration_profile_id(Option<String>)`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::set_configuration_profile_id):<br>required: **true**<br><p>The configuration profile ID.</p><br>
    ///   - [`configuration_version(impl Into<String>)`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::configuration_version) / [`set_configuration_version(Option<String>)`](crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::set_configuration_version):<br>required: **true**<br><p>The version of the configuration to validate.</p><br>
    /// - On success, responds with [`ValidateConfigurationOutput`](crate::operation::validate_configuration::ValidateConfigurationOutput)
    /// - On failure, responds with [`SdkError<ValidateConfigurationError>`](crate::operation::validate_configuration::ValidateConfigurationError)
    pub fn validate_configuration(&self) -> crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder {
        crate::operation::validate_configuration::builders::ValidateConfigurationFluentBuilder::new(self.handle.clone())
    }
}
