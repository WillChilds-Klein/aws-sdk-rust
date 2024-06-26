// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RequestSenderId`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sender_id(impl Into<String>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::sender_id) / [`set_sender_id(Option<String>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::set_sender_id):<br>required: **true**<br><p>The sender ID string to request.</p><br>
    ///   - [`iso_country_code(impl Into<String>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::iso_country_code) / [`set_iso_country_code(Option<String>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::set_iso_country_code):<br>required: **true**<br><p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p><br>
    ///   - [`message_types(MessageType)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::message_types) / [`set_message_types(Option<Vec::<MessageType>>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::set_message_types):<br>required: **false**<br><p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p><br>
    ///   - [`deletion_protection_enabled(bool)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::deletion_protection_enabled) / [`set_deletion_protection_enabled(Option<bool>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::set_deletion_protection_enabled):<br>required: **false**<br><p>By default this is set to false. When set to true the sender ID can't be deleted.</p><br>
    ///   - [`tags(Tag)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::set_tags):<br>required: **false**<br><p>An array of tags (key and value pairs) to associate with the sender ID.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don't specify a client token, a randomly generated token is used for the request to ensure idempotency.</p><br>
    /// - On success, responds with [`RequestSenderIdOutput`](crate::operation::request_sender_id::RequestSenderIdOutput) with field(s):
    ///   - [`sender_id_arn(String)`](crate::operation::request_sender_id::RequestSenderIdOutput::sender_id_arn): <p>The Amazon Resource Name (ARN) associated with the SenderId.</p>
    ///   - [`sender_id(String)`](crate::operation::request_sender_id::RequestSenderIdOutput::sender_id): <p>The sender ID that was requested.</p>
    ///   - [`iso_country_code(String)`](crate::operation::request_sender_id::RequestSenderIdOutput::iso_country_code): <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    ///   - [`message_types(Vec::<MessageType>)`](crate::operation::request_sender_id::RequestSenderIdOutput::message_types): <p>The type of message. Valid values are TRANSACTIONAL for messages that are critical or time-sensitive and PROMOTIONAL for messages that aren't critical or time-sensitive.</p>
    ///   - [`monthly_leasing_price(String)`](crate::operation::request_sender_id::RequestSenderIdOutput::monthly_leasing_price): <p>The monthly price, in US dollars, to lease the sender ID.</p>
    ///   - [`deletion_protection_enabled(bool)`](crate::operation::request_sender_id::RequestSenderIdOutput::deletion_protection_enabled): <p>By default this is set to false. When set to true the sender ID can't be deleted.</p>
    ///   - [`registered(bool)`](crate::operation::request_sender_id::RequestSenderIdOutput::registered): <p>True if the sender ID is registered.</p>
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::request_sender_id::RequestSenderIdOutput::tags): <p>An array of tags (key and value pairs) to associate with the sender ID.</p>
    /// - On failure, responds with [`SdkError<RequestSenderIdError>`](crate::operation::request_sender_id::RequestSenderIdError)
    pub fn request_sender_id(&self) -> crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder {
        crate::operation::request_sender_id::builders::RequestSenderIdFluentBuilder::new(self.handle.clone())
    }
}
