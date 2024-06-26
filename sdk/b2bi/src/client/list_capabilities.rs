// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCapabilities`](crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder::set_next_token):<br>required: **false**<br><p>When additional results are obtained from the command, a <code>NextToken</code> parameter is returned in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional resources.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder::set_max_results):<br>required: **false**<br><p>Specifies the maximum number of capabilities to return.</p><br>
    /// - On success, responds with [`ListCapabilitiesOutput`](crate::operation::list_capabilities::ListCapabilitiesOutput) with field(s):
    ///   - [`capabilities(Vec::<CapabilitySummary>)`](crate::operation::list_capabilities::ListCapabilitiesOutput::capabilities): <p>Returns one or more capabilities associated with this partnership.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_capabilities::ListCapabilitiesOutput::next_token): <p>When additional results are obtained from the command, a <code>NextToken</code> parameter is returned in the output. You can then pass the <code>NextToken</code> parameter in a subsequent command to continue listing additional resources.</p>
    /// - On failure, responds with [`SdkError<ListCapabilitiesError>`](crate::operation::list_capabilities::ListCapabilitiesError)
    pub fn list_capabilities(&self) -> crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder {
        crate::operation::list_capabilities::builders::ListCapabilitiesFluentBuilder::new(self.handle.clone())
    }
}
