// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableAllFeatures`](crate::operation::enable_all_features::builders::EnableAllFeaturesFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::enable_all_features::builders::EnableAllFeaturesFluentBuilder::send) it.
    /// - On success, responds with [`EnableAllFeaturesOutput`](crate::operation::enable_all_features::EnableAllFeaturesOutput) with field(s):
    ///   - [`handshake(Option<Handshake>)`](crate::operation::enable_all_features::EnableAllFeaturesOutput::handshake): <p>A structure that contains details about the handshake created to support this request to enable all features in the organization.</p>
    /// - On failure, responds with [`SdkError<EnableAllFeaturesError>`](crate::operation::enable_all_features::EnableAllFeaturesError)
    pub fn enable_all_features(&self) -> crate::operation::enable_all_features::builders::EnableAllFeaturesFluentBuilder {
        crate::operation::enable_all_features::builders::EnableAllFeaturesFluentBuilder::new(self.handle.clone())
    }
}
