// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDelegatedAdminAccount`](crate::operation::get_delegated_admin_account::builders::GetDelegatedAdminAccountFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_delegated_admin_account::builders::GetDelegatedAdminAccountFluentBuilder::send) it.
    /// - On success, responds with [`GetDelegatedAdminAccountOutput`](crate::operation::get_delegated_admin_account::GetDelegatedAdminAccountOutput) with field(s):
    ///   - [`delegated_admin(Option<DelegatedAdmin>)`](crate::operation::get_delegated_admin_account::GetDelegatedAdminAccountOutput::delegated_admin): <p>The Amazon Web Services account ID of the Amazon Inspector delegated administrator.</p>
    /// - On failure, responds with [`SdkError<GetDelegatedAdminAccountError>`](crate::operation::get_delegated_admin_account::GetDelegatedAdminAccountError)
    pub fn get_delegated_admin_account(&self) -> crate::operation::get_delegated_admin_account::builders::GetDelegatedAdminAccountFluentBuilder {
        crate::operation::get_delegated_admin_account::builders::GetDelegatedAdminAccountFluentBuilder::new(self.handle.clone())
    }
}
