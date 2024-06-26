// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFolderContents`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`authentication_token(impl Into<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::authentication_token) / [`set_authentication_token(Option<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_authentication_token):<br>required: **false**<br><p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p><br>
    ///   - [`folder_id(impl Into<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::folder_id) / [`set_folder_id(Option<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_folder_id):<br>required: **true**<br><p>The ID of the folder.</p><br>
    ///   - [`sort(ResourceSortType)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::sort) / [`set_sort(Option<ResourceSortType>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_sort):<br>required: **false**<br><p>The sorting criteria.</p><br>
    ///   - [`order(OrderType)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::order) / [`set_order(Option<OrderType>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_order):<br>required: **false**<br><p>The order for the contents of the folder.</p><br>
    ///   - [`limit(i32)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of items to return with this call.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_marker):<br>required: **false**<br><p>The marker for the next set of results. This marker was received from a previous call.</p><br>
    ///   - [`r#type(FolderContentType)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::type) / [`set_type(Option<FolderContentType>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_type):<br>required: **false**<br><p>The type of items.</p><br>
    ///   - [`include(impl Into<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::include) / [`set_include(Option<String>)`](crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::set_include):<br>required: **false**<br><p>The contents to include. Specify "INITIALIZED" to include initialized documents.</p><br>
    /// - On success, responds with [`DescribeFolderContentsOutput`](crate::operation::describe_folder_contents::DescribeFolderContentsOutput) with field(s):
    ///   - [`folders(Option<Vec::<FolderMetadata>>)`](crate::operation::describe_folder_contents::DescribeFolderContentsOutput::folders): <p>The subfolders in the specified folder.</p>
    ///   - [`documents(Option<Vec::<DocumentMetadata>>)`](crate::operation::describe_folder_contents::DescribeFolderContentsOutput::documents): <p>The documents in the specified folder.</p>
    ///   - [`marker(Option<String>)`](crate::operation::describe_folder_contents::DescribeFolderContentsOutput::marker): <p>The marker to use when requesting the next set of results. If there are no additional results, the string is empty.</p>
    /// - On failure, responds with [`SdkError<DescribeFolderContentsError>`](crate::operation::describe_folder_contents::DescribeFolderContentsError)
    pub fn describe_folder_contents(&self) -> crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder {
        crate::operation::describe_folder_contents::builders::DescribeFolderContentsFluentBuilder::new(self.handle.clone())
    }
}
