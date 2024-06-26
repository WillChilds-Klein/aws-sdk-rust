// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRevealConfiguration`](crate::operation::get_reveal_configuration::builders::GetRevealConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_reveal_configuration::builders::GetRevealConfigurationFluentBuilder::send) it.
    /// - On success, responds with [`GetRevealConfigurationOutput`](crate::operation::get_reveal_configuration::GetRevealConfigurationOutput) with field(s):
    ///   - [`configuration(Option<RevealConfiguration>)`](crate::operation::get_reveal_configuration::GetRevealConfigurationOutput::configuration): <p>The KMS key that's used to encrypt the sensitive data, and the status of the configuration for the Amazon Macie account.</p>
    ///   - [`retrieval_configuration(Option<RetrievalConfiguration>)`](crate::operation::get_reveal_configuration::GetRevealConfigurationOutput::retrieval_configuration): <p>The access method and settings that are used to retrieve the sensitive data.</p>
    /// - On failure, responds with [`SdkError<GetRevealConfigurationError>`](crate::operation::get_reveal_configuration::GetRevealConfigurationError)
    pub fn get_reveal_configuration(&self) -> crate::operation::get_reveal_configuration::builders::GetRevealConfigurationFluentBuilder {
        crate::operation::get_reveal_configuration::builders::GetRevealConfigurationFluentBuilder::new(self.handle.clone())
    }
}
