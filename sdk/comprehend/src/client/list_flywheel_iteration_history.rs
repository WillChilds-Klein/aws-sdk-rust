// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFlywheelIterationHistory`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`flywheel_arn(impl Into<String>)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::flywheel_arn) / [`set_flywheel_arn(Option<String>)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::set_flywheel_arn):<br>required: **true**<br><p>The ARN of the flywheel.</p><br>
    ///   - [`filter(FlywheelIterationFilter)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::filter) / [`set_filter(Option<FlywheelIterationFilter>)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::set_filter):<br>required: **false**<br><p>Filter the flywheel iteration history based on creation time.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::set_next_token):<br>required: **false**<br><p>Next token</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::set_max_results):<br>required: **false**<br><p>Maximum number of iteration history results to return</p><br>
    /// - On success, responds with [`ListFlywheelIterationHistoryOutput`](crate::operation::list_flywheel_iteration_history::ListFlywheelIterationHistoryOutput) with field(s):
    ///   - [`flywheel_iteration_properties_list(Option<Vec::<FlywheelIterationProperties>>)`](crate::operation::list_flywheel_iteration_history::ListFlywheelIterationHistoryOutput::flywheel_iteration_properties_list): <p>List of flywheel iteration properties</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_flywheel_iteration_history::ListFlywheelIterationHistoryOutput::next_token): <p>Next token</p>
    /// - On failure, responds with [`SdkError<ListFlywheelIterationHistoryError>`](crate::operation::list_flywheel_iteration_history::ListFlywheelIterationHistoryError)
    pub fn list_flywheel_iteration_history(
        &self,
    ) -> crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder {
        crate::operation::list_flywheel_iteration_history::builders::ListFlywheelIterationHistoryFluentBuilder::new(self.handle.clone())
    }
}
