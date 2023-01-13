// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateRescoreExecutionPlan`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_rescore_execution_plan`](crate::client::Client::create_rescore_execution_plan).
///
/// See [`crate::client::fluent_builders::CreateRescoreExecutionPlan`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateRescoreExecutionPlan {
    _private: (),
}
impl CreateRescoreExecutionPlan {
    /// Creates a new builder-style object to manufacture [`CreateRescoreExecutionPlanInput`](crate::input::CreateRescoreExecutionPlanInput).
    pub fn builder() -> crate::input::create_rescore_execution_plan_input::Builder {
        crate::input::create_rescore_execution_plan_input::Builder::default()
    }
    /// Creates a new `CreateRescoreExecutionPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRescoreExecutionPlan {
    type Output = std::result::Result<
        crate::output::CreateRescoreExecutionPlanOutput,
        crate::error::CreateRescoreExecutionPlanError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_rescore_execution_plan_error(response)
        } else {
            crate::operation_deser::parse_create_rescore_execution_plan_response(response)
        }
    }
}

/// Operation shape for `DeleteRescoreExecutionPlan`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_rescore_execution_plan`](crate::client::Client::delete_rescore_execution_plan).
///
/// See [`crate::client::fluent_builders::DeleteRescoreExecutionPlan`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRescoreExecutionPlan {
    _private: (),
}
impl DeleteRescoreExecutionPlan {
    /// Creates a new builder-style object to manufacture [`DeleteRescoreExecutionPlanInput`](crate::input::DeleteRescoreExecutionPlanInput).
    pub fn builder() -> crate::input::delete_rescore_execution_plan_input::Builder {
        crate::input::delete_rescore_execution_plan_input::Builder::default()
    }
    /// Creates a new `DeleteRescoreExecutionPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRescoreExecutionPlan {
    type Output = std::result::Result<
        crate::output::DeleteRescoreExecutionPlanOutput,
        crate::error::DeleteRescoreExecutionPlanError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_rescore_execution_plan_error(response)
        } else {
            crate::operation_deser::parse_delete_rescore_execution_plan_response(response)
        }
    }
}

/// Operation shape for `DescribeRescoreExecutionPlan`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_rescore_execution_plan`](crate::client::Client::describe_rescore_execution_plan).
///
/// See [`crate::client::fluent_builders::DescribeRescoreExecutionPlan`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeRescoreExecutionPlan {
    _private: (),
}
impl DescribeRescoreExecutionPlan {
    /// Creates a new builder-style object to manufacture [`DescribeRescoreExecutionPlanInput`](crate::input::DescribeRescoreExecutionPlanInput).
    pub fn builder() -> crate::input::describe_rescore_execution_plan_input::Builder {
        crate::input::describe_rescore_execution_plan_input::Builder::default()
    }
    /// Creates a new `DescribeRescoreExecutionPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRescoreExecutionPlan {
    type Output = std::result::Result<
        crate::output::DescribeRescoreExecutionPlanOutput,
        crate::error::DescribeRescoreExecutionPlanError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_rescore_execution_plan_error(response)
        } else {
            crate::operation_deser::parse_describe_rescore_execution_plan_response(response)
        }
    }
}

/// Operation shape for `ListRescoreExecutionPlans`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_rescore_execution_plans`](crate::client::Client::list_rescore_execution_plans).
///
/// See [`crate::client::fluent_builders::ListRescoreExecutionPlans`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRescoreExecutionPlans {
    _private: (),
}
impl ListRescoreExecutionPlans {
    /// Creates a new builder-style object to manufacture [`ListRescoreExecutionPlansInput`](crate::input::ListRescoreExecutionPlansInput).
    pub fn builder() -> crate::input::list_rescore_execution_plans_input::Builder {
        crate::input::list_rescore_execution_plans_input::Builder::default()
    }
    /// Creates a new `ListRescoreExecutionPlans` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRescoreExecutionPlans {
    type Output = std::result::Result<
        crate::output::ListRescoreExecutionPlansOutput,
        crate::error::ListRescoreExecutionPlansError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_rescore_execution_plans_error(response)
        } else {
            crate::operation_deser::parse_list_rescore_execution_plans_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `Rescore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`rescore`](crate::client::Client::rescore).
///
/// See [`crate::client::fluent_builders::Rescore`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct Rescore {
    _private: (),
}
impl Rescore {
    /// Creates a new builder-style object to manufacture [`RescoreInput`](crate::input::RescoreInput).
    pub fn builder() -> crate::input::rescore_input::Builder {
        crate::input::rescore_input::Builder::default()
    }
    /// Creates a new `Rescore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Rescore {
    type Output = std::result::Result<crate::output::RescoreOutput, crate::error::RescoreError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_rescore_error(response)
        } else {
            crate::operation_deser::parse_rescore_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateRescoreExecutionPlan`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_rescore_execution_plan`](crate::client::Client::update_rescore_execution_plan).
///
/// See [`crate::client::fluent_builders::UpdateRescoreExecutionPlan`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateRescoreExecutionPlan {
    _private: (),
}
impl UpdateRescoreExecutionPlan {
    /// Creates a new builder-style object to manufacture [`UpdateRescoreExecutionPlanInput`](crate::input::UpdateRescoreExecutionPlanInput).
    pub fn builder() -> crate::input::update_rescore_execution_plan_input::Builder {
        crate::input::update_rescore_execution_plan_input::Builder::default()
    }
    /// Creates a new `UpdateRescoreExecutionPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRescoreExecutionPlan {
    type Output = std::result::Result<
        crate::output::UpdateRescoreExecutionPlanOutput,
        crate::error::UpdateRescoreExecutionPlanError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_rescore_execution_plan_error(response)
        } else {
            crate::operation_deser::parse_update_rescore_execution_plan_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
