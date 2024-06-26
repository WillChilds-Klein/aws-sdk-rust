// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ActivateOrganizationsAccess`](crate::operation::activate_organizations_access::builders::ActivateOrganizationsAccessFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::activate_organizations_access::builders::ActivateOrganizationsAccessFluentBuilder::send) it.
    /// - On success, responds with [`ActivateOrganizationsAccessOutput`](crate::operation::activate_organizations_access::ActivateOrganizationsAccessOutput)
    /// - On failure, responds with [`SdkError<ActivateOrganizationsAccessError>`](crate::operation::activate_organizations_access::ActivateOrganizationsAccessError)
    pub fn activate_organizations_access(
        &self,
    ) -> crate::operation::activate_organizations_access::builders::ActivateOrganizationsAccessFluentBuilder {
        crate::operation::activate_organizations_access::builders::ActivateOrganizationsAccessFluentBuilder::new(self.handle.clone())
    }
}
