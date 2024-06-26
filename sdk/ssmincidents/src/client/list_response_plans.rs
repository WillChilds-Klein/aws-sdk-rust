// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListResponsePlans`](crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of response plans per page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token for the next set of items to return. (You received this token from a previous call.)</p><br>
    /// - On success, responds with [`ListResponsePlansOutput`](crate::operation::list_response_plans::ListResponsePlansOutput) with field(s):
    ///   - [`response_plan_summaries(Vec::<ResponsePlanSummary>)`](crate::operation::list_response_plans::ListResponsePlansOutput::response_plan_summaries): <p>Details of each response plan.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_response_plans::ListResponsePlansOutput::next_token): <p>The pagination token to use when requesting the next set of items. If there are no additional items to return, the string is null.</p>
    /// - On failure, responds with [`SdkError<ListResponsePlansError>`](crate::operation::list_response_plans::ListResponsePlansError)
    pub fn list_response_plans(&self) -> crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder {
        crate::operation::list_response_plans::builders::ListResponsePlansFluentBuilder::new(self.handle.clone())
    }
}
