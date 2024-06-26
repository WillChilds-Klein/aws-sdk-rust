// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGroup`](crate::operation::update_group::builders::UpdateGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_name(impl Into<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::set_group_name):<br>required: **false**<br><p>Don't use this parameter. Use <code>Group</code> instead.</p><br>
    ///   - [`group(impl Into<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::group) / [`set_group(Option<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::set_group):<br>required: **false**<br><p>The name or the ARN of the resource group to modify.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::set_description):<br>required: **false**<br><p>The new description that you want to update the resource group with. Descriptions can contain letters, numbers, hyphens, underscores, periods, and spaces.</p><br>
    /// - On success, responds with [`UpdateGroupOutput`](crate::operation::update_group::UpdateGroupOutput) with field(s):
    ///   - [`group(Option<Group>)`](crate::operation::update_group::UpdateGroupOutput::group): <p>The update description of the resource group.</p>
    /// - On failure, responds with [`SdkError<UpdateGroupError>`](crate::operation::update_group::UpdateGroupError)
    pub fn update_group(&self) -> crate::operation::update_group::builders::UpdateGroupFluentBuilder {
        crate::operation::update_group::builders::UpdateGroupFluentBuilder::new(self.handle.clone())
    }
}
