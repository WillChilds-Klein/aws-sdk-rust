// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateProfile`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`profile_id(impl Into<String>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::profile_id) / [`set_profile_id(Option<String>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::set_profile_id):<br>required: **true**<br><p>ID of the Profile.</p><br>
    ///   - [`resource_id(impl Into<String>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::set_resource_id):<br>required: **true**<br><p>The ID of the VPC.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::set_name):<br>required: **true**<br><p>A name for the association.</p><br>
    ///   - [`tags(Tag)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::set_tags):<br>required: **false**<br><p>A list of the tag keys and values that you want to identify the Profile association.</p><br>
    /// - On success, responds with [`AssociateProfileOutput`](crate::operation::associate_profile::AssociateProfileOutput) with field(s):
    ///   - [`profile_association(Option<ProfileAssociation>)`](crate::operation::associate_profile::AssociateProfileOutput::profile_association): <p>The association that you just created. The association has an ID that you can use to identify it in other requests, like update and delete.</p>
    /// - On failure, responds with [`SdkError<AssociateProfileError>`](crate::operation::associate_profile::AssociateProfileError)
    pub fn associate_profile(&self) -> crate::operation::associate_profile::builders::AssociateProfileFluentBuilder {
        crate::operation::associate_profile::builders::AssociateProfileFluentBuilder::new(self.handle.clone())
    }
}
