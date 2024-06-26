// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListProfiles`](crate::operation::list_profiles::builders::ListProfilesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_profiles::builders::ListProfilesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_profiles::builders::ListProfilesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_profiles::builders::ListProfilesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of objects that you want to return for this request. If more objects are available, in the response, a <code>NextToken</code> value, which you can use in a subsequent call to get the next batch of objects, is provided.</p> <p>If you don't specify a value for <code>MaxResults</code>, up to 100 objects are returned.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_profiles::builders::ListProfilesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_profiles::builders::ListProfilesFluentBuilder::set_next_token):<br>required: **false**<br><p>For the first call to this list request, omit this value.</p> <p>When you request a list of objects, at most the number of objects specified by <code>MaxResults</code> is returned. If more objects are available for retrieval, a <code>NextToken</code> value is returned in the response. To retrieve the next batch of objects, use the token that was returned for the prior request in your next request.</p><br>
    /// - On success, responds with [`ListProfilesOutput`](crate::operation::list_profiles::ListProfilesOutput) with field(s):
    ///   - [`profile_summaries(Option<Vec::<ProfileSummary>>)`](crate::operation::list_profiles::ListProfilesOutput::profile_summaries): <p>Summary information about the Profiles.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_profiles::ListProfilesOutput::next_token): <p>If more than <code>MaxResults</code> resource associations match the specified criteria, you can submit another <code>ListProfiles</code> request to get the next group of results. In the next request, specify the value of <code>NextToken</code> from the previous response.</p>
    /// - On failure, responds with [`SdkError<ListProfilesError>`](crate::operation::list_profiles::ListProfilesError)
    pub fn list_profiles(&self) -> crate::operation::list_profiles::builders::ListProfilesFluentBuilder {
        crate::operation::list_profiles::builders::ListProfilesFluentBuilder::new(self.handle.clone())
    }
}
