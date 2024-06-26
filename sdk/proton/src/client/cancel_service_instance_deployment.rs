// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelServiceInstanceDeployment`](crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`service_instance_name(impl Into<String>)`](crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder::service_instance_name) / [`set_service_instance_name(Option<String>)`](crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder::set_service_instance_name):<br>required: **true**<br><p>The name of the service instance with the deployment to cancel.</p><br>
    ///   - [`service_name(impl Into<String>)`](crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder::service_name) / [`set_service_name(Option<String>)`](crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder::set_service_name):<br>required: **true**<br><p>The name of the service with the service instance deployment to cancel.</p><br>
    /// - On success, responds with [`CancelServiceInstanceDeploymentOutput`](crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput) with field(s):
    ///   - [`service_instance(Option<ServiceInstance>)`](crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentOutput::service_instance): <p>The service instance summary data that's returned by Proton.</p>
    /// - On failure, responds with [`SdkError<CancelServiceInstanceDeploymentError>`](crate::operation::cancel_service_instance_deployment::CancelServiceInstanceDeploymentError)
    pub fn cancel_service_instance_deployment(
        &self,
    ) -> crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder {
        crate::operation::cancel_service_instance_deployment::builders::CancelServiceInstanceDeploymentFluentBuilder::new(self.handle.clone())
    }
}
