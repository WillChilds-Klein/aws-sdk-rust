// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`Rescore`](crate::operation::rescore::builders::RescoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rescore_execution_plan_id(impl Into<String>)`](crate::operation::rescore::builders::RescoreFluentBuilder::rescore_execution_plan_id) / [`set_rescore_execution_plan_id(Option<String>)`](crate::operation::rescore::builders::RescoreFluentBuilder::set_rescore_execution_plan_id):<br>required: **true**<br><p>The identifier of the rescore execution plan. A rescore execution plan is an Amazon Kendra Intelligent Ranking resource used for provisioning the <code>Rescore</code> API.</p><br>
    ///   - [`search_query(impl Into<String>)`](crate::operation::rescore::builders::RescoreFluentBuilder::search_query) / [`set_search_query(Option<String>)`](crate::operation::rescore::builders::RescoreFluentBuilder::set_search_query):<br>required: **true**<br><p>The input query from the search service.</p><br>
    ///   - [`documents(Document)`](crate::operation::rescore::builders::RescoreFluentBuilder::documents) / [`set_documents(Option<Vec::<Document>>)`](crate::operation::rescore::builders::RescoreFluentBuilder::set_documents):<br>required: **true**<br><p>The list of documents for Amazon Kendra Intelligent Ranking to rescore or rank on.</p><br>
    /// - On success, responds with [`RescoreOutput`](crate::operation::rescore::RescoreOutput) with field(s):
    ///   - [`rescore_id(Option<String>)`](crate::operation::rescore::RescoreOutput::rescore_id): <p>The identifier associated with the scores that Amazon Kendra Intelligent Ranking gives to the results. Amazon Kendra Intelligent Ranking rescores or re-ranks the results for the search service.</p>
    ///   - [`result_items(Option<Vec::<RescoreResultItem>>)`](crate::operation::rescore::RescoreOutput::result_items): <p>A list of result items for documents with new relevancy scores. The results are in descending order.</p>
    /// - On failure, responds with [`SdkError<RescoreError>`](crate::operation::rescore::RescoreError)
    pub fn rescore(&self) -> crate::operation::rescore::builders::RescoreFluentBuilder {
        crate::operation::rescore::builders::RescoreFluentBuilder::new(self.handle.clone())
    }
}
