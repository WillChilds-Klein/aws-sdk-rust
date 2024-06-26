// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteInstance`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::set_instance_id):<br>required: **true**<br><p>The instance ID.</p><br>
    ///   - [`delete_elastic_ip(bool)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::delete_elastic_ip) / [`set_delete_elastic_ip(Option<bool>)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::set_delete_elastic_ip):<br>required: **false**<br><p>Whether to delete the instance Elastic IP address.</p><br>
    ///   - [`delete_volumes(bool)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::delete_volumes) / [`set_delete_volumes(Option<bool>)`](crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::set_delete_volumes):<br>required: **false**<br><p>Whether to delete the instance's Amazon EBS volumes.</p><br>
    /// - On success, responds with [`DeleteInstanceOutput`](crate::operation::delete_instance::DeleteInstanceOutput)
    /// - On failure, responds with [`SdkError<DeleteInstanceError>`](crate::operation::delete_instance::DeleteInstanceError)
    pub fn delete_instance(&self) -> crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder {
        crate::operation::delete_instance::builders::DeleteInstanceFluentBuilder::new(self.handle.clone())
    }
}
