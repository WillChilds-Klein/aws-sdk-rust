// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateReferenceStore`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::set_name):<br>required: **true**<br><p>A name for the store.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::set_description):<br>required: **false**<br><p>A description for the store.</p><br>
    ///   - [`sse_config(SseConfig)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::sse_config) / [`set_sse_config(Option<SseConfig>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::set_sse_config):<br>required: **false**<br><p>Server-side encryption (SSE) settings for the store.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::set_tags):<br>required: **false**<br><p>Tags for the store.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::set_client_token):<br>required: **false**<br><p>To ensure that requests don't run multiple times, specify a unique token for each request.</p><br>
    /// - On success, responds with [`CreateReferenceStoreOutput`](crate::operation::create_reference_store::CreateReferenceStoreOutput) with field(s):
    ///   - [`id(String)`](crate::operation::create_reference_store::CreateReferenceStoreOutput::id): <p>The store's ID.</p>
    ///   - [`arn(String)`](crate::operation::create_reference_store::CreateReferenceStoreOutput::arn): <p>The store's ARN.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_reference_store::CreateReferenceStoreOutput::name): <p>The store's name.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_reference_store::CreateReferenceStoreOutput::description): <p>The store's description.</p>
    ///   - [`sse_config(Option<SseConfig>)`](crate::operation::create_reference_store::CreateReferenceStoreOutput::sse_config): <p>The store's SSE settings.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::create_reference_store::CreateReferenceStoreOutput::creation_time): <p>When the store was created.</p>
    /// - On failure, responds with [`SdkError<CreateReferenceStoreError>`](crate::operation::create_reference_store::CreateReferenceStoreError)
    pub fn create_reference_store(&self) -> crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder {
        crate::operation::create_reference_store::builders::CreateReferenceStoreFluentBuilder::new(self.handle.clone())
    }
}
