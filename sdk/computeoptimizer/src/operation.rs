// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteRecommendationPreferences`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_recommendation_preferences`](crate::client::Client::delete_recommendation_preferences).
///
/// See [`crate::client::fluent_builders::DeleteRecommendationPreferences`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRecommendationPreferences {
    _private: (),
}
impl DeleteRecommendationPreferences {
    /// Creates a new builder-style object to manufacture [`DeleteRecommendationPreferencesInput`](crate::input::DeleteRecommendationPreferencesInput).
    pub fn builder() -> crate::input::delete_recommendation_preferences_input::Builder {
        crate::input::delete_recommendation_preferences_input::Builder::default()
    }
    /// Creates a new `DeleteRecommendationPreferences` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRecommendationPreferences {
    type Output = std::result::Result<
        crate::output::DeleteRecommendationPreferencesOutput,
        crate::error::DeleteRecommendationPreferencesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_recommendation_preferences_error(response)
        } else {
            crate::operation_deser::parse_delete_recommendation_preferences_response(response)
        }
    }
}

/// Operation shape for `DescribeRecommendationExportJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_recommendation_export_jobs`](crate::client::Client::describe_recommendation_export_jobs).
///
/// See [`crate::client::fluent_builders::DescribeRecommendationExportJobs`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeRecommendationExportJobs {
    _private: (),
}
impl DescribeRecommendationExportJobs {
    /// Creates a new builder-style object to manufacture [`DescribeRecommendationExportJobsInput`](crate::input::DescribeRecommendationExportJobsInput).
    pub fn builder() -> crate::input::describe_recommendation_export_jobs_input::Builder {
        crate::input::describe_recommendation_export_jobs_input::Builder::default()
    }
    /// Creates a new `DescribeRecommendationExportJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRecommendationExportJobs {
    type Output = std::result::Result<
        crate::output::DescribeRecommendationExportJobsOutput,
        crate::error::DescribeRecommendationExportJobsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_recommendation_export_jobs_error(response)
        } else {
            crate::operation_deser::parse_describe_recommendation_export_jobs_response(response)
        }
    }
}

/// Operation shape for `ExportAutoScalingGroupRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_auto_scaling_group_recommendations`](crate::client::Client::export_auto_scaling_group_recommendations).
///
/// See [`crate::client::fluent_builders::ExportAutoScalingGroupRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportAutoScalingGroupRecommendations {
    _private: (),
}
impl ExportAutoScalingGroupRecommendations {
    /// Creates a new builder-style object to manufacture [`ExportAutoScalingGroupRecommendationsInput`](crate::input::ExportAutoScalingGroupRecommendationsInput).
    pub fn builder() -> crate::input::export_auto_scaling_group_recommendations_input::Builder {
        crate::input::export_auto_scaling_group_recommendations_input::Builder::default()
    }
    /// Creates a new `ExportAutoScalingGroupRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportAutoScalingGroupRecommendations {
    type Output = std::result::Result<
        crate::output::ExportAutoScalingGroupRecommendationsOutput,
        crate::error::ExportAutoScalingGroupRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_auto_scaling_group_recommendations_error(response)
        } else {
            crate::operation_deser::parse_export_auto_scaling_group_recommendations_response(
                response,
            )
        }
    }
}

/// Operation shape for `ExportEBSVolumeRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_ebs_volume_recommendations`](crate::client::Client::export_ebs_volume_recommendations).
///
/// See [`crate::client::fluent_builders::ExportEBSVolumeRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportEBSVolumeRecommendations {
    _private: (),
}
impl ExportEBSVolumeRecommendations {
    /// Creates a new builder-style object to manufacture [`ExportEbsVolumeRecommendationsInput`](crate::input::ExportEbsVolumeRecommendationsInput).
    pub fn builder() -> crate::input::export_ebs_volume_recommendations_input::Builder {
        crate::input::export_ebs_volume_recommendations_input::Builder::default()
    }
    /// Creates a new `ExportEBSVolumeRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportEBSVolumeRecommendations {
    type Output = std::result::Result<
        crate::output::ExportEbsVolumeRecommendationsOutput,
        crate::error::ExportEBSVolumeRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_ebs_volume_recommendations_error(response)
        } else {
            crate::operation_deser::parse_export_ebs_volume_recommendations_response(response)
        }
    }
}

/// Operation shape for `ExportEC2InstanceRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_ec2_instance_recommendations`](crate::client::Client::export_ec2_instance_recommendations).
///
/// See [`crate::client::fluent_builders::ExportEC2InstanceRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportEC2InstanceRecommendations {
    _private: (),
}
impl ExportEC2InstanceRecommendations {
    /// Creates a new builder-style object to manufacture [`ExportEc2InstanceRecommendationsInput`](crate::input::ExportEc2InstanceRecommendationsInput).
    pub fn builder() -> crate::input::export_ec2_instance_recommendations_input::Builder {
        crate::input::export_ec2_instance_recommendations_input::Builder::default()
    }
    /// Creates a new `ExportEC2InstanceRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportEC2InstanceRecommendations {
    type Output = std::result::Result<
        crate::output::ExportEc2InstanceRecommendationsOutput,
        crate::error::ExportEC2InstanceRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_ec2_instance_recommendations_error(response)
        } else {
            crate::operation_deser::parse_export_ec2_instance_recommendations_response(response)
        }
    }
}

/// Operation shape for `ExportECSServiceRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_ecs_service_recommendations`](crate::client::Client::export_ecs_service_recommendations).
///
/// See [`crate::client::fluent_builders::ExportECSServiceRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportECSServiceRecommendations {
    _private: (),
}
impl ExportECSServiceRecommendations {
    /// Creates a new builder-style object to manufacture [`ExportEcsServiceRecommendationsInput`](crate::input::ExportEcsServiceRecommendationsInput).
    pub fn builder() -> crate::input::export_ecs_service_recommendations_input::Builder {
        crate::input::export_ecs_service_recommendations_input::Builder::default()
    }
    /// Creates a new `ExportECSServiceRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportECSServiceRecommendations {
    type Output = std::result::Result<
        crate::output::ExportEcsServiceRecommendationsOutput,
        crate::error::ExportECSServiceRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_ecs_service_recommendations_error(response)
        } else {
            crate::operation_deser::parse_export_ecs_service_recommendations_response(response)
        }
    }
}

/// Operation shape for `ExportLambdaFunctionRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_lambda_function_recommendations`](crate::client::Client::export_lambda_function_recommendations).
///
/// See [`crate::client::fluent_builders::ExportLambdaFunctionRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportLambdaFunctionRecommendations {
    _private: (),
}
impl ExportLambdaFunctionRecommendations {
    /// Creates a new builder-style object to manufacture [`ExportLambdaFunctionRecommendationsInput`](crate::input::ExportLambdaFunctionRecommendationsInput).
    pub fn builder() -> crate::input::export_lambda_function_recommendations_input::Builder {
        crate::input::export_lambda_function_recommendations_input::Builder::default()
    }
    /// Creates a new `ExportLambdaFunctionRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportLambdaFunctionRecommendations {
    type Output = std::result::Result<
        crate::output::ExportLambdaFunctionRecommendationsOutput,
        crate::error::ExportLambdaFunctionRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_lambda_function_recommendations_error(response)
        } else {
            crate::operation_deser::parse_export_lambda_function_recommendations_response(response)
        }
    }
}

/// Operation shape for `GetAutoScalingGroupRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_auto_scaling_group_recommendations`](crate::client::Client::get_auto_scaling_group_recommendations).
///
/// See [`crate::client::fluent_builders::GetAutoScalingGroupRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAutoScalingGroupRecommendations {
    _private: (),
}
impl GetAutoScalingGroupRecommendations {
    /// Creates a new builder-style object to manufacture [`GetAutoScalingGroupRecommendationsInput`](crate::input::GetAutoScalingGroupRecommendationsInput).
    pub fn builder() -> crate::input::get_auto_scaling_group_recommendations_input::Builder {
        crate::input::get_auto_scaling_group_recommendations_input::Builder::default()
    }
    /// Creates a new `GetAutoScalingGroupRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAutoScalingGroupRecommendations {
    type Output = std::result::Result<
        crate::output::GetAutoScalingGroupRecommendationsOutput,
        crate::error::GetAutoScalingGroupRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_auto_scaling_group_recommendations_error(response)
        } else {
            crate::operation_deser::parse_get_auto_scaling_group_recommendations_response(response)
        }
    }
}

/// Operation shape for `GetEBSVolumeRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_ebs_volume_recommendations`](crate::client::Client::get_ebs_volume_recommendations).
///
/// See [`crate::client::fluent_builders::GetEBSVolumeRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEBSVolumeRecommendations {
    _private: (),
}
impl GetEBSVolumeRecommendations {
    /// Creates a new builder-style object to manufacture [`GetEbsVolumeRecommendationsInput`](crate::input::GetEbsVolumeRecommendationsInput).
    pub fn builder() -> crate::input::get_ebs_volume_recommendations_input::Builder {
        crate::input::get_ebs_volume_recommendations_input::Builder::default()
    }
    /// Creates a new `GetEBSVolumeRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEBSVolumeRecommendations {
    type Output = std::result::Result<
        crate::output::GetEbsVolumeRecommendationsOutput,
        crate::error::GetEBSVolumeRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_ebs_volume_recommendations_error(response)
        } else {
            crate::operation_deser::parse_get_ebs_volume_recommendations_response(response)
        }
    }
}

/// Operation shape for `GetEC2InstanceRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_ec2_instance_recommendations`](crate::client::Client::get_ec2_instance_recommendations).
///
/// See [`crate::client::fluent_builders::GetEC2InstanceRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEC2InstanceRecommendations {
    _private: (),
}
impl GetEC2InstanceRecommendations {
    /// Creates a new builder-style object to manufacture [`GetEc2InstanceRecommendationsInput`](crate::input::GetEc2InstanceRecommendationsInput).
    pub fn builder() -> crate::input::get_ec2_instance_recommendations_input::Builder {
        crate::input::get_ec2_instance_recommendations_input::Builder::default()
    }
    /// Creates a new `GetEC2InstanceRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEC2InstanceRecommendations {
    type Output = std::result::Result<
        crate::output::GetEc2InstanceRecommendationsOutput,
        crate::error::GetEC2InstanceRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_ec2_instance_recommendations_error(response)
        } else {
            crate::operation_deser::parse_get_ec2_instance_recommendations_response(response)
        }
    }
}

/// Operation shape for `GetEC2RecommendationProjectedMetrics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_ec2_recommendation_projected_metrics`](crate::client::Client::get_ec2_recommendation_projected_metrics).
///
/// See [`crate::client::fluent_builders::GetEC2RecommendationProjectedMetrics`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEC2RecommendationProjectedMetrics {
    _private: (),
}
impl GetEC2RecommendationProjectedMetrics {
    /// Creates a new builder-style object to manufacture [`GetEc2RecommendationProjectedMetricsInput`](crate::input::GetEc2RecommendationProjectedMetricsInput).
    pub fn builder() -> crate::input::get_ec2_recommendation_projected_metrics_input::Builder {
        crate::input::get_ec2_recommendation_projected_metrics_input::Builder::default()
    }
    /// Creates a new `GetEC2RecommendationProjectedMetrics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEC2RecommendationProjectedMetrics {
    type Output = std::result::Result<
        crate::output::GetEc2RecommendationProjectedMetricsOutput,
        crate::error::GetEC2RecommendationProjectedMetricsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_ec2_recommendation_projected_metrics_error(response)
        } else {
            crate::operation_deser::parse_get_ec2_recommendation_projected_metrics_response(
                response,
            )
        }
    }
}

/// Operation shape for `GetECSServiceRecommendationProjectedMetrics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_ecs_service_recommendation_projected_metrics`](crate::client::Client::get_ecs_service_recommendation_projected_metrics).
///
/// See [`crate::client::fluent_builders::GetECSServiceRecommendationProjectedMetrics`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetECSServiceRecommendationProjectedMetrics {
    _private: (),
}
impl GetECSServiceRecommendationProjectedMetrics {
    /// Creates a new builder-style object to manufacture [`GetEcsServiceRecommendationProjectedMetricsInput`](crate::input::GetEcsServiceRecommendationProjectedMetricsInput).
    pub fn builder() -> crate::input::get_ecs_service_recommendation_projected_metrics_input::Builder
    {
        crate::input::get_ecs_service_recommendation_projected_metrics_input::Builder::default()
    }
    /// Creates a new `GetECSServiceRecommendationProjectedMetrics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse
    for GetECSServiceRecommendationProjectedMetrics
{
    type Output = std::result::Result<
        crate::output::GetEcsServiceRecommendationProjectedMetricsOutput,
        crate::error::GetECSServiceRecommendationProjectedMetricsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_ecs_service_recommendation_projected_metrics_error(
                response,
            )
        } else {
            crate::operation_deser::parse_get_ecs_service_recommendation_projected_metrics_response(
                response,
            )
        }
    }
}

/// Operation shape for `GetECSServiceRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_ecs_service_recommendations`](crate::client::Client::get_ecs_service_recommendations).
///
/// See [`crate::client::fluent_builders::GetECSServiceRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetECSServiceRecommendations {
    _private: (),
}
impl GetECSServiceRecommendations {
    /// Creates a new builder-style object to manufacture [`GetEcsServiceRecommendationsInput`](crate::input::GetEcsServiceRecommendationsInput).
    pub fn builder() -> crate::input::get_ecs_service_recommendations_input::Builder {
        crate::input::get_ecs_service_recommendations_input::Builder::default()
    }
    /// Creates a new `GetECSServiceRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetECSServiceRecommendations {
    type Output = std::result::Result<
        crate::output::GetEcsServiceRecommendationsOutput,
        crate::error::GetECSServiceRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_ecs_service_recommendations_error(response)
        } else {
            crate::operation_deser::parse_get_ecs_service_recommendations_response(response)
        }
    }
}

/// Operation shape for `GetEffectiveRecommendationPreferences`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_effective_recommendation_preferences`](crate::client::Client::get_effective_recommendation_preferences).
///
/// See [`crate::client::fluent_builders::GetEffectiveRecommendationPreferences`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEffectiveRecommendationPreferences {
    _private: (),
}
impl GetEffectiveRecommendationPreferences {
    /// Creates a new builder-style object to manufacture [`GetEffectiveRecommendationPreferencesInput`](crate::input::GetEffectiveRecommendationPreferencesInput).
    pub fn builder() -> crate::input::get_effective_recommendation_preferences_input::Builder {
        crate::input::get_effective_recommendation_preferences_input::Builder::default()
    }
    /// Creates a new `GetEffectiveRecommendationPreferences` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEffectiveRecommendationPreferences {
    type Output = std::result::Result<
        crate::output::GetEffectiveRecommendationPreferencesOutput,
        crate::error::GetEffectiveRecommendationPreferencesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_effective_recommendation_preferences_error(response)
        } else {
            crate::operation_deser::parse_get_effective_recommendation_preferences_response(
                response,
            )
        }
    }
}

/// Operation shape for `GetEnrollmentStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_enrollment_status`](crate::client::Client::get_enrollment_status).
///
/// See [`crate::client::fluent_builders::GetEnrollmentStatus`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEnrollmentStatus {
    _private: (),
}
impl GetEnrollmentStatus {
    /// Creates a new builder-style object to manufacture [`GetEnrollmentStatusInput`](crate::input::GetEnrollmentStatusInput).
    pub fn builder() -> crate::input::get_enrollment_status_input::Builder {
        crate::input::get_enrollment_status_input::Builder::default()
    }
    /// Creates a new `GetEnrollmentStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEnrollmentStatus {
    type Output = std::result::Result<
        crate::output::GetEnrollmentStatusOutput,
        crate::error::GetEnrollmentStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_enrollment_status_error(response)
        } else {
            crate::operation_deser::parse_get_enrollment_status_response(response)
        }
    }
}

/// Operation shape for `GetEnrollmentStatusesForOrganization`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_enrollment_statuses_for_organization`](crate::client::Client::get_enrollment_statuses_for_organization).
///
/// See [`crate::client::fluent_builders::GetEnrollmentStatusesForOrganization`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetEnrollmentStatusesForOrganization {
    _private: (),
}
impl GetEnrollmentStatusesForOrganization {
    /// Creates a new builder-style object to manufacture [`GetEnrollmentStatusesForOrganizationInput`](crate::input::GetEnrollmentStatusesForOrganizationInput).
    pub fn builder() -> crate::input::get_enrollment_statuses_for_organization_input::Builder {
        crate::input::get_enrollment_statuses_for_organization_input::Builder::default()
    }
    /// Creates a new `GetEnrollmentStatusesForOrganization` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEnrollmentStatusesForOrganization {
    type Output = std::result::Result<
        crate::output::GetEnrollmentStatusesForOrganizationOutput,
        crate::error::GetEnrollmentStatusesForOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_enrollment_statuses_for_organization_error(response)
        } else {
            crate::operation_deser::parse_get_enrollment_statuses_for_organization_response(
                response,
            )
        }
    }
}

/// Operation shape for `GetLambdaFunctionRecommendations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_lambda_function_recommendations`](crate::client::Client::get_lambda_function_recommendations).
///
/// See [`crate::client::fluent_builders::GetLambdaFunctionRecommendations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetLambdaFunctionRecommendations {
    _private: (),
}
impl GetLambdaFunctionRecommendations {
    /// Creates a new builder-style object to manufacture [`GetLambdaFunctionRecommendationsInput`](crate::input::GetLambdaFunctionRecommendationsInput).
    pub fn builder() -> crate::input::get_lambda_function_recommendations_input::Builder {
        crate::input::get_lambda_function_recommendations_input::Builder::default()
    }
    /// Creates a new `GetLambdaFunctionRecommendations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLambdaFunctionRecommendations {
    type Output = std::result::Result<
        crate::output::GetLambdaFunctionRecommendationsOutput,
        crate::error::GetLambdaFunctionRecommendationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lambda_function_recommendations_error(response)
        } else {
            crate::operation_deser::parse_get_lambda_function_recommendations_response(response)
        }
    }
}

/// Operation shape for `GetRecommendationPreferences`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_recommendation_preferences`](crate::client::Client::get_recommendation_preferences).
///
/// See [`crate::client::fluent_builders::GetRecommendationPreferences`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRecommendationPreferences {
    _private: (),
}
impl GetRecommendationPreferences {
    /// Creates a new builder-style object to manufacture [`GetRecommendationPreferencesInput`](crate::input::GetRecommendationPreferencesInput).
    pub fn builder() -> crate::input::get_recommendation_preferences_input::Builder {
        crate::input::get_recommendation_preferences_input::Builder::default()
    }
    /// Creates a new `GetRecommendationPreferences` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecommendationPreferences {
    type Output = std::result::Result<
        crate::output::GetRecommendationPreferencesOutput,
        crate::error::GetRecommendationPreferencesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_recommendation_preferences_error(response)
        } else {
            crate::operation_deser::parse_get_recommendation_preferences_response(response)
        }
    }
}

/// Operation shape for `GetRecommendationSummaries`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_recommendation_summaries`](crate::client::Client::get_recommendation_summaries).
///
/// See [`crate::client::fluent_builders::GetRecommendationSummaries`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRecommendationSummaries {
    _private: (),
}
impl GetRecommendationSummaries {
    /// Creates a new builder-style object to manufacture [`GetRecommendationSummariesInput`](crate::input::GetRecommendationSummariesInput).
    pub fn builder() -> crate::input::get_recommendation_summaries_input::Builder {
        crate::input::get_recommendation_summaries_input::Builder::default()
    }
    /// Creates a new `GetRecommendationSummaries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecommendationSummaries {
    type Output = std::result::Result<
        crate::output::GetRecommendationSummariesOutput,
        crate::error::GetRecommendationSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_recommendation_summaries_error(response)
        } else {
            crate::operation_deser::parse_get_recommendation_summaries_response(response)
        }
    }
}

/// Operation shape for `PutRecommendationPreferences`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_recommendation_preferences`](crate::client::Client::put_recommendation_preferences).
///
/// See [`crate::client::fluent_builders::PutRecommendationPreferences`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutRecommendationPreferences {
    _private: (),
}
impl PutRecommendationPreferences {
    /// Creates a new builder-style object to manufacture [`PutRecommendationPreferencesInput`](crate::input::PutRecommendationPreferencesInput).
    pub fn builder() -> crate::input::put_recommendation_preferences_input::Builder {
        crate::input::put_recommendation_preferences_input::Builder::default()
    }
    /// Creates a new `PutRecommendationPreferences` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecommendationPreferences {
    type Output = std::result::Result<
        crate::output::PutRecommendationPreferencesOutput,
        crate::error::PutRecommendationPreferencesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_recommendation_preferences_error(response)
        } else {
            crate::operation_deser::parse_put_recommendation_preferences_response(response)
        }
    }
}

/// Operation shape for `UpdateEnrollmentStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_enrollment_status`](crate::client::Client::update_enrollment_status).
///
/// See [`crate::client::fluent_builders::UpdateEnrollmentStatus`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateEnrollmentStatus {
    _private: (),
}
impl UpdateEnrollmentStatus {
    /// Creates a new builder-style object to manufacture [`UpdateEnrollmentStatusInput`](crate::input::UpdateEnrollmentStatusInput).
    pub fn builder() -> crate::input::update_enrollment_status_input::Builder {
        crate::input::update_enrollment_status_input::Builder::default()
    }
    /// Creates a new `UpdateEnrollmentStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEnrollmentStatus {
    type Output = std::result::Result<
        crate::output::UpdateEnrollmentStatusOutput,
        crate::error::UpdateEnrollmentStatusError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_enrollment_status_error(response)
        } else {
            crate::operation_deser::parse_update_enrollment_status_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
