// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePackageGroup`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::domain) / [`set_domain(Option<String>)`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::set_domain):<br>required: **true**<br><p>The name of the domain that contains the package group.</p><br>
    ///   - [`domain_owner(impl Into<String>)`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::domain_owner) / [`set_domain_owner(Option<String>)`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::set_domain_owner):<br>required: **false**<br><p>The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces.</p><br>
    ///   - [`package_group(impl Into<String>)`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::package_group) / [`set_package_group(Option<String>)`](crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::set_package_group):<br>required: **true**<br><p>The pattern of the requested package group.</p><br>
    /// - On success, responds with [`DescribePackageGroupOutput`](crate::operation::describe_package_group::DescribePackageGroupOutput) with field(s):
    ///   - [`package_group(Option<PackageGroupDescription>)`](crate::operation::describe_package_group::DescribePackageGroupOutput::package_group): <p>A <a href="https://docs.aws.amazon.com/codeartifact/latest/APIReference/API_PackageGroupDescription.html">PackageGroupDescription</a> object that contains information about the requested package group.</p>
    /// - On failure, responds with [`SdkError<DescribePackageGroupError>`](crate::operation::describe_package_group::DescribePackageGroupError)
    pub fn describe_package_group(&self) -> crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder {
        crate::operation::describe_package_group::builders::DescribePackageGroupFluentBuilder::new(self.handle.clone())
    }
}
