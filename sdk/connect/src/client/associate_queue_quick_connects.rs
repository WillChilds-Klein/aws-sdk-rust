// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateQueueQuickConnects`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::set_instance_id):<br>required: **true**<br><p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p><br>
    ///   - [`queue_id(impl Into<String>)`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::queue_id) / [`set_queue_id(Option<String>)`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::set_queue_id):<br>required: **true**<br><p>The identifier for the queue.</p><br>
    ///   - [`quick_connect_ids(impl Into<String>)`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::quick_connect_ids) / [`set_quick_connect_ids(Option<Vec::<String>>)`](crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::set_quick_connect_ids):<br>required: **true**<br><p>The quick connects to associate with this queue.</p><br>
    /// - On success, responds with [`AssociateQueueQuickConnectsOutput`](crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsOutput)
    /// - On failure, responds with [`SdkError<AssociateQueueQuickConnectsError>`](crate::operation::associate_queue_quick_connects::AssociateQueueQuickConnectsError)
    pub fn associate_queue_quick_connects(
        &self,
    ) -> crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder {
        crate::operation::associate_queue_quick_connects::builders::AssociateQueueQuickConnectsFluentBuilder::new(self.handle.clone())
    }
}
