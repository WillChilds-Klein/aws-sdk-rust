// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateIntegration`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workload_id(impl Into<String>)`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::workload_id) / [`set_workload_id(Option<String>)`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::set_workload_id):<br>required: **true**<br><p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p><br>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::set_client_request_token):<br>required: **true**<br><p>A unique case-sensitive string used to ensure that this request is idempotent (executes only once).</p> <p>You should not reuse the same token for other requests. If you retry a request with the same client request token and the same parameters after the original request has completed successfully, the result of the original request is returned.</p><important>  <p>This token is listed as required, however, if you do not specify it, the Amazon Web Services SDKs automatically generate one for you. If you are not using the Amazon Web Services SDK or the CLI, you must provide this token or the request will fail.</p> </important><br>
    ///   - [`integrating_service(IntegratingService)`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::integrating_service) / [`set_integrating_service(Option<IntegratingService>)`](crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::set_integrating_service):<br>required: **true**<br><p>Which integrated service to update.</p><br>
    /// - On success, responds with [`UpdateIntegrationOutput`](crate::operation::update_integration::UpdateIntegrationOutput)
    /// - On failure, responds with [`SdkError<UpdateIntegrationError>`](crate::operation::update_integration::UpdateIntegrationError)
    pub fn update_integration(&self) -> crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder {
        crate::operation::update_integration::builders::UpdateIntegrationFluentBuilder::new(self.handle.clone())
    }
}
