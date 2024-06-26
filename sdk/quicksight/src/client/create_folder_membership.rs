// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateFolderMembership`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID for the Amazon Web Services account that contains the folder.</p><br>
    ///   - [`folder_id(impl Into<String>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::folder_id) / [`set_folder_id(Option<String>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::set_folder_id):<br>required: **true**<br><p>The ID of the folder.</p><br>
    ///   - [`member_id(impl Into<String>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::member_id) / [`set_member_id(Option<String>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::set_member_id):<br>required: **true**<br><p>The ID of the asset that you want to add to the folder.</p><br>
    ///   - [`member_type(MemberType)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::member_type) / [`set_member_type(Option<MemberType>)`](crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::set_member_type):<br>required: **true**<br><p>The member type of the asset that you want to add to a folder.</p><br>
    /// - On success, responds with [`CreateFolderMembershipOutput`](crate::operation::create_folder_membership::CreateFolderMembershipOutput) with field(s):
    ///   - [`status(i32)`](crate::operation::create_folder_membership::CreateFolderMembershipOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`folder_member(Option<FolderMember>)`](crate::operation::create_folder_membership::CreateFolderMembershipOutput::folder_member): <p>Information about the member in the folder.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::create_folder_membership::CreateFolderMembershipOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<CreateFolderMembershipError>`](crate::operation::create_folder_membership::CreateFolderMembershipError)
    pub fn create_folder_membership(&self) -> crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder {
        crate::operation::create_folder_membership::builders::CreateFolderMembershipFluentBuilder::new(self.handle.clone())
    }
}
