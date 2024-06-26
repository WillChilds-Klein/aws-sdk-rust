// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLifecycleExecution`](crate::operation::get_lifecycle_execution::builders::GetLifecycleExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`lifecycle_execution_id(impl Into<String>)`](crate::operation::get_lifecycle_execution::builders::GetLifecycleExecutionFluentBuilder::lifecycle_execution_id) / [`set_lifecycle_execution_id(Option<String>)`](crate::operation::get_lifecycle_execution::builders::GetLifecycleExecutionFluentBuilder::set_lifecycle_execution_id):<br>required: **true**<br><p>Use the unique identifier for a runtime instance of the lifecycle policy to get runtime details.</p><br>
    /// - On success, responds with [`GetLifecycleExecutionOutput`](crate::operation::get_lifecycle_execution::GetLifecycleExecutionOutput) with field(s):
    ///   - [`lifecycle_execution(Option<LifecycleExecution>)`](crate::operation::get_lifecycle_execution::GetLifecycleExecutionOutput::lifecycle_execution): <p>Runtime details for the specified runtime instance of the lifecycle policy.</p>
    /// - On failure, responds with [`SdkError<GetLifecycleExecutionError>`](crate::operation::get_lifecycle_execution::GetLifecycleExecutionError)
    pub fn get_lifecycle_execution(&self) -> crate::operation::get_lifecycle_execution::builders::GetLifecycleExecutionFluentBuilder {
        crate::operation::get_lifecycle_execution::builders::GetLifecycleExecutionFluentBuilder::new(self.handle.clone())
    }
}
