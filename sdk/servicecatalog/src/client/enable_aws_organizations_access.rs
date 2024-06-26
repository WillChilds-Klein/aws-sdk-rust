// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableAWSOrganizationsAccess`](crate::operation::enable_aws_organizations_access::builders::EnableAWSOrganizationsAccessFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::enable_aws_organizations_access::builders::EnableAWSOrganizationsAccessFluentBuilder::send) it.
    /// - On success, responds with [`EnableAwsOrganizationsAccessOutput`](crate::operation::enable_aws_organizations_access::EnableAwsOrganizationsAccessOutput)
    /// - On failure, responds with [`SdkError<EnableAWSOrganizationsAccessError>`](crate::operation::enable_aws_organizations_access::EnableAWSOrganizationsAccessError)
    pub fn enable_aws_organizations_access(
        &self,
    ) -> crate::operation::enable_aws_organizations_access::builders::EnableAWSOrganizationsAccessFluentBuilder {
        crate::operation::enable_aws_organizations_access::builders::EnableAWSOrganizationsAccessFluentBuilder::new(self.handle.clone())
    }
}
