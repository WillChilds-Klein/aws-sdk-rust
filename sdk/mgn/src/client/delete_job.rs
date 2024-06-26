// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteJob`](crate::operation::delete_job::builders::DeleteJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`job_id(impl Into<String>)`](crate::operation::delete_job::builders::DeleteJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::delete_job::builders::DeleteJobFluentBuilder::set_job_id):<br>required: **true**<br><p>Request to delete Job from service by Job ID.</p><br>
    ///   - [`account_id(impl Into<String>)`](crate::operation::delete_job::builders::DeleteJobFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_job::builders::DeleteJobFluentBuilder::set_account_id):<br>required: **false**<br><p>Request to delete Job from service by Account ID.</p><br>
    /// - On success, responds with [`DeleteJobOutput`](crate::operation::delete_job::DeleteJobOutput)
    /// - On failure, responds with [`SdkError<DeleteJobError>`](crate::operation::delete_job::DeleteJobError)
    pub fn delete_job(&self) -> crate::operation::delete_job::builders::DeleteJobFluentBuilder {
        crate::operation::delete_job::builders::DeleteJobFluentBuilder::new(self.handle.clone())
    }
}
