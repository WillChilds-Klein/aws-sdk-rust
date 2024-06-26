// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListMembers`](crate::operation::list_members::builders::ListMembersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_members::builders::ListMembersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`only_associated(bool)`](crate::operation::list_members::builders::ListMembersFluentBuilder::only_associated) / [`set_only_associated(Option<bool>)`](crate::operation::list_members::builders::ListMembersFluentBuilder::set_only_associated):<br>required: **false**<br><p>Specifies whether to list only currently associated members if <code>True</code> or to list all members within the organization if <code>False</code>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_members::builders::ListMembersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_members::builders::ListMembersFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results the response can return. If your request would return more than the maximum the response will return a <code>nextToken</code> value, use this value when you call the action again to get the remaining results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_members::builders::ListMembersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_members::builders::ListMembersFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. If your response returns more than the <code>maxResults</code> maximum value it will also return a <code>nextToken</code> value. For subsequent calls, use the <code>nextToken</code> value returned from the previous request to continue listing results after the first page.</p><br>
    /// - On success, responds with [`ListMembersOutput`](crate::operation::list_members::ListMembersOutput) with field(s):
    ///   - [`members(Option<Vec::<Member>>)`](crate::operation::list_members::ListMembersOutput::members): <p>An object that contains details for each member account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_members::ListMembersOutput::next_token): <p>The pagination parameter to be used on the next list operation to retrieve more items.</p>
    /// - On failure, responds with [`SdkError<ListMembersError>`](crate::operation::list_members::ListMembersError)
    pub fn list_members(&self) -> crate::operation::list_members::builders::ListMembersFluentBuilder {
        crate::operation::list_members::builders::ListMembersFluentBuilder::new(self.handle.clone())
    }
}
