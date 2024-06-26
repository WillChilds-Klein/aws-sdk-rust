// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateFieldLevelEncryptionProfile`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`field_level_encryption_profile_config(FieldLevelEncryptionProfileConfig)`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::field_level_encryption_profile_config) / [`set_field_level_encryption_profile_config(Option<FieldLevelEncryptionProfileConfig>)`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::set_field_level_encryption_profile_config):<br>required: **true**<br><p>Request to update a field-level encryption profile.</p><br>
    ///   - [`id(impl Into<String>)`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::set_id):<br>required: **true**<br><p>The ID of the field-level encryption profile request.</p><br>
    ///   - [`if_match(impl Into<String>)`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::if_match) / [`set_if_match(Option<String>)`](crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::set_if_match):<br>required: **false**<br><p>The value of the <code>ETag</code> header that you received when retrieving the profile identity to update. For example: <code>E2QWRUHAPOMQZL</code>.</p><br>
    /// - On success, responds with [`UpdateFieldLevelEncryptionProfileOutput`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileOutput) with field(s):
    ///   - [`field_level_encryption_profile(Option<FieldLevelEncryptionProfile>)`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileOutput::field_level_encryption_profile): <p>Return the results of updating the profile.</p>
    ///   - [`e_tag(Option<String>)`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileOutput::e_tag): <p>The result of the field-level encryption profile request.</p>
    /// - On failure, responds with [`SdkError<UpdateFieldLevelEncryptionProfileError>`](crate::operation::update_field_level_encryption_profile::UpdateFieldLevelEncryptionProfileError)
    pub fn update_field_level_encryption_profile(
        &self,
    ) -> crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder {
        crate::operation::update_field_level_encryption_profile::builders::UpdateFieldLevelEncryptionProfileFluentBuilder::new(self.handle.clone())
    }
}
