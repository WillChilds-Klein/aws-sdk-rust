// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteApplicationInputProcessingConfiguration`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::application_name) / [`set_application_name(Option<String>)`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::set_application_name):<br>required: **true**<br><p>The Kinesis Analytics application name.</p><br>
    ///   - [`current_application_version_id(i64)`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::current_application_version_id) / [`set_current_application_version_id(Option<i64>)`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::set_current_application_version_id):<br>required: **true**<br><p>The version ID of the Kinesis Analytics application.</p><br>
    ///   - [`input_id(impl Into<String>)`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::input_id) / [`set_input_id(Option<String>)`](crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::set_input_id):<br>required: **true**<br><p>The ID of the input configuration from which to delete the input processing configuration. You can get a list of the input IDs for an application by using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p><br>
    /// - On success, responds with [`DeleteApplicationInputProcessingConfigurationOutput`](crate::operation::delete_application_input_processing_configuration::DeleteApplicationInputProcessingConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteApplicationInputProcessingConfigurationError>`](crate::operation::delete_application_input_processing_configuration::DeleteApplicationInputProcessingConfigurationError)
    pub fn delete_application_input_processing_configuration(
        &self,
    ) -> crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder
    {
        crate::operation::delete_application_input_processing_configuration::builders::DeleteApplicationInputProcessingConfigurationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
