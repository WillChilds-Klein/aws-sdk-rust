// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateWorkspaceAlias`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::set_workspace_id):<br>required: **true**<br><p>The ID of the workspace to update.</p><br>
    ///   - [`alias(impl Into<String>)`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::alias) / [`set_alias(Option<String>)`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::set_alias):<br>required: **false**<br><p>The new alias for the workspace. It does not need to be unique.</p> <p>Amazon Managed Service for Prometheus will automatically strip any blank spaces from the beginning and end of the alias that you specify.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique identifier that you can provide to ensure the idempotency of the request. Case-sensitive.</p><br>
    /// - On success, responds with [`UpdateWorkspaceAliasOutput`](crate::operation::update_workspace_alias::UpdateWorkspaceAliasOutput)
    /// - On failure, responds with [`SdkError<UpdateWorkspaceAliasError>`](crate::operation::update_workspace_alias::UpdateWorkspaceAliasError)
    pub fn update_workspace_alias(&self) -> crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder {
        crate::operation::update_workspace_alias::builders::UpdateWorkspaceAliasFluentBuilder::new(self.handle.clone())
    }
}
