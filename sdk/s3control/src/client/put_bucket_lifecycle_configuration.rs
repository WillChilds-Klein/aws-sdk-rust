// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketLifecycleConfiguration`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::set_account_id):<br>required: **true**<br><p>The Amazon Web Services account ID of the Outposts bucket.</p><br>
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the bucket for which to set the configuration.</p><br>
    ///   - [`lifecycle_configuration(LifecycleConfiguration)`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::lifecycle_configuration) / [`set_lifecycle_configuration(Option<LifecycleConfiguration>)`](crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::set_lifecycle_configuration):<br>required: **false**<br><p>Container for lifecycle rules. You can add as many as 1,000 rules.</p><br>
    /// - On success, responds with [`PutBucketLifecycleConfigurationOutput`](crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutBucketLifecycleConfigurationError>`](crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError)
    pub fn put_bucket_lifecycle_configuration(
        &self,
    ) -> crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder {
        crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationFluentBuilder::new(self.handle.clone())
    }
}
