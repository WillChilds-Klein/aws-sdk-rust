// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteChatControlsConfiguration`](crate::operation::delete_chat_controls_configuration::builders::DeleteChatControlsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::delete_chat_controls_configuration::builders::DeleteChatControlsConfigurationFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::delete_chat_controls_configuration::builders::DeleteChatControlsConfigurationFluentBuilder::set_application_id):<br>required: **true**<br><p>The identifier of the application the chat controls have been configured for.</p><br>
    /// - On success, responds with [`DeleteChatControlsConfigurationOutput`](crate::operation::delete_chat_controls_configuration::DeleteChatControlsConfigurationOutput)
    /// - On failure, responds with [`SdkError<DeleteChatControlsConfigurationError>`](crate::operation::delete_chat_controls_configuration::DeleteChatControlsConfigurationError)
    pub fn delete_chat_controls_configuration(
        &self,
    ) -> crate::operation::delete_chat_controls_configuration::builders::DeleteChatControlsConfigurationFluentBuilder {
        crate::operation::delete_chat_controls_configuration::builders::DeleteChatControlsConfigurationFluentBuilder::new(self.handle.clone())
    }
}
