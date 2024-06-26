// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetStorageConfiguration`](crate::operation::get_storage_configuration::builders::GetStorageConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::get_storage_configuration::builders::GetStorageConfigurationFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::get_storage_configuration::builders::GetStorageConfigurationFluentBuilder::set_arn):<br>required: **true**<br><p>ARN of the storage configuration to be retrieved.</p><br>
    /// - On success, responds with [`GetStorageConfigurationOutput`](crate::operation::get_storage_configuration::GetStorageConfigurationOutput) with field(s):
    ///   - [`storage_configuration(Option<StorageConfiguration>)`](crate::operation::get_storage_configuration::GetStorageConfigurationOutput::storage_configuration): <p>The StorageConfiguration that was returned.</p>
    /// - On failure, responds with [`SdkError<GetStorageConfigurationError>`](crate::operation::get_storage_configuration::GetStorageConfigurationError)
    pub fn get_storage_configuration(&self) -> crate::operation::get_storage_configuration::builders::GetStorageConfigurationFluentBuilder {
        crate::operation::get_storage_configuration::builders::GetStorageConfigurationFluentBuilder::new(self.handle.clone())
    }
}
