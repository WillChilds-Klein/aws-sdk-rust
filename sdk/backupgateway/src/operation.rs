// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateGatewayToServer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`associate_gateway_to_server`](crate::client::Client::associate_gateway_to_server).
///
/// See [`crate::client::fluent_builders::AssociateGatewayToServer`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateGatewayToServer {
    _private: (),
}
impl AssociateGatewayToServer {
    /// Creates a new builder-style object to manufacture [`AssociateGatewayToServerInput`](crate::input::AssociateGatewayToServerInput).
    pub fn builder() -> crate::input::associate_gateway_to_server_input::Builder {
        crate::input::associate_gateway_to_server_input::Builder::default()
    }
    /// Creates a new `AssociateGatewayToServer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateGatewayToServer {
    type Output = std::result::Result<
        crate::output::AssociateGatewayToServerOutput,
        crate::error::AssociateGatewayToServerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_gateway_to_server_error(response)
        } else {
            crate::operation_deser::parse_associate_gateway_to_server_response(response)
        }
    }
}

/// Operation shape for `CreateGateway`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_gateway`](crate::client::Client::create_gateway).
///
/// See [`crate::client::fluent_builders::CreateGateway`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateGateway {
    _private: (),
}
impl CreateGateway {
    /// Creates a new builder-style object to manufacture [`CreateGatewayInput`](crate::input::CreateGatewayInput).
    pub fn builder() -> crate::input::create_gateway_input::Builder {
        crate::input::create_gateway_input::Builder::default()
    }
    /// Creates a new `CreateGateway` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateGateway {
    type Output =
        std::result::Result<crate::output::CreateGatewayOutput, crate::error::CreateGatewayError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_gateway_error(response)
        } else {
            crate::operation_deser::parse_create_gateway_response(response)
        }
    }
}

/// Operation shape for `DeleteGateway`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_gateway`](crate::client::Client::delete_gateway).
///
/// See [`crate::client::fluent_builders::DeleteGateway`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteGateway {
    _private: (),
}
impl DeleteGateway {
    /// Creates a new builder-style object to manufacture [`DeleteGatewayInput`](crate::input::DeleteGatewayInput).
    pub fn builder() -> crate::input::delete_gateway_input::Builder {
        crate::input::delete_gateway_input::Builder::default()
    }
    /// Creates a new `DeleteGateway` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteGateway {
    type Output =
        std::result::Result<crate::output::DeleteGatewayOutput, crate::error::DeleteGatewayError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_gateway_error(response)
        } else {
            crate::operation_deser::parse_delete_gateway_response(response)
        }
    }
}

/// Operation shape for `DeleteHypervisor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_hypervisor`](crate::client::Client::delete_hypervisor).
///
/// See [`crate::client::fluent_builders::DeleteHypervisor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteHypervisor {
    _private: (),
}
impl DeleteHypervisor {
    /// Creates a new builder-style object to manufacture [`DeleteHypervisorInput`](crate::input::DeleteHypervisorInput).
    pub fn builder() -> crate::input::delete_hypervisor_input::Builder {
        crate::input::delete_hypervisor_input::Builder::default()
    }
    /// Creates a new `DeleteHypervisor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteHypervisor {
    type Output = std::result::Result<
        crate::output::DeleteHypervisorOutput,
        crate::error::DeleteHypervisorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_hypervisor_error(response)
        } else {
            crate::operation_deser::parse_delete_hypervisor_response(response)
        }
    }
}

/// Operation shape for `DisassociateGatewayFromServer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`disassociate_gateway_from_server`](crate::client::Client::disassociate_gateway_from_server).
///
/// See [`crate::client::fluent_builders::DisassociateGatewayFromServer`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateGatewayFromServer {
    _private: (),
}
impl DisassociateGatewayFromServer {
    /// Creates a new builder-style object to manufacture [`DisassociateGatewayFromServerInput`](crate::input::DisassociateGatewayFromServerInput).
    pub fn builder() -> crate::input::disassociate_gateway_from_server_input::Builder {
        crate::input::disassociate_gateway_from_server_input::Builder::default()
    }
    /// Creates a new `DisassociateGatewayFromServer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateGatewayFromServer {
    type Output = std::result::Result<
        crate::output::DisassociateGatewayFromServerOutput,
        crate::error::DisassociateGatewayFromServerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_gateway_from_server_error(response)
        } else {
            crate::operation_deser::parse_disassociate_gateway_from_server_response(response)
        }
    }
}

/// Operation shape for `GetBandwidthRateLimitSchedule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_bandwidth_rate_limit_schedule`](crate::client::Client::get_bandwidth_rate_limit_schedule).
///
/// See [`crate::client::fluent_builders::GetBandwidthRateLimitSchedule`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetBandwidthRateLimitSchedule {
    _private: (),
}
impl GetBandwidthRateLimitSchedule {
    /// Creates a new builder-style object to manufacture [`GetBandwidthRateLimitScheduleInput`](crate::input::GetBandwidthRateLimitScheduleInput).
    pub fn builder() -> crate::input::get_bandwidth_rate_limit_schedule_input::Builder {
        crate::input::get_bandwidth_rate_limit_schedule_input::Builder::default()
    }
    /// Creates a new `GetBandwidthRateLimitSchedule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetBandwidthRateLimitSchedule {
    type Output = std::result::Result<
        crate::output::GetBandwidthRateLimitScheduleOutput,
        crate::error::GetBandwidthRateLimitScheduleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_bandwidth_rate_limit_schedule_error(response)
        } else {
            crate::operation_deser::parse_get_bandwidth_rate_limit_schedule_response(response)
        }
    }
}

/// Operation shape for `GetGateway`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_gateway`](crate::client::Client::get_gateway).
///
/// See [`crate::client::fluent_builders::GetGateway`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetGateway {
    _private: (),
}
impl GetGateway {
    /// Creates a new builder-style object to manufacture [`GetGatewayInput`](crate::input::GetGatewayInput).
    pub fn builder() -> crate::input::get_gateway_input::Builder {
        crate::input::get_gateway_input::Builder::default()
    }
    /// Creates a new `GetGateway` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGateway {
    type Output =
        std::result::Result<crate::output::GetGatewayOutput, crate::error::GetGatewayError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_gateway_error(response)
        } else {
            crate::operation_deser::parse_get_gateway_response(response)
        }
    }
}

/// Operation shape for `GetHypervisor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_hypervisor`](crate::client::Client::get_hypervisor).
///
/// See [`crate::client::fluent_builders::GetHypervisor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetHypervisor {
    _private: (),
}
impl GetHypervisor {
    /// Creates a new builder-style object to manufacture [`GetHypervisorInput`](crate::input::GetHypervisorInput).
    pub fn builder() -> crate::input::get_hypervisor_input::Builder {
        crate::input::get_hypervisor_input::Builder::default()
    }
    /// Creates a new `GetHypervisor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetHypervisor {
    type Output =
        std::result::Result<crate::output::GetHypervisorOutput, crate::error::GetHypervisorError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_hypervisor_error(response)
        } else {
            crate::operation_deser::parse_get_hypervisor_response(response)
        }
    }
}

/// Operation shape for `GetHypervisorPropertyMappings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_hypervisor_property_mappings`](crate::client::Client::get_hypervisor_property_mappings).
///
/// See [`crate::client::fluent_builders::GetHypervisorPropertyMappings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetHypervisorPropertyMappings {
    _private: (),
}
impl GetHypervisorPropertyMappings {
    /// Creates a new builder-style object to manufacture [`GetHypervisorPropertyMappingsInput`](crate::input::GetHypervisorPropertyMappingsInput).
    pub fn builder() -> crate::input::get_hypervisor_property_mappings_input::Builder {
        crate::input::get_hypervisor_property_mappings_input::Builder::default()
    }
    /// Creates a new `GetHypervisorPropertyMappings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetHypervisorPropertyMappings {
    type Output = std::result::Result<
        crate::output::GetHypervisorPropertyMappingsOutput,
        crate::error::GetHypervisorPropertyMappingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_hypervisor_property_mappings_error(response)
        } else {
            crate::operation_deser::parse_get_hypervisor_property_mappings_response(response)
        }
    }
}

/// Operation shape for `GetVirtualMachine`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_virtual_machine`](crate::client::Client::get_virtual_machine).
///
/// See [`crate::client::fluent_builders::GetVirtualMachine`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetVirtualMachine {
    _private: (),
}
impl GetVirtualMachine {
    /// Creates a new builder-style object to manufacture [`GetVirtualMachineInput`](crate::input::GetVirtualMachineInput).
    pub fn builder() -> crate::input::get_virtual_machine_input::Builder {
        crate::input::get_virtual_machine_input::Builder::default()
    }
    /// Creates a new `GetVirtualMachine` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetVirtualMachine {
    type Output = std::result::Result<
        crate::output::GetVirtualMachineOutput,
        crate::error::GetVirtualMachineError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_virtual_machine_error(response)
        } else {
            crate::operation_deser::parse_get_virtual_machine_response(response)
        }
    }
}

/// Operation shape for `ImportHypervisorConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`import_hypervisor_configuration`](crate::client::Client::import_hypervisor_configuration).
///
/// See [`crate::client::fluent_builders::ImportHypervisorConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportHypervisorConfiguration {
    _private: (),
}
impl ImportHypervisorConfiguration {
    /// Creates a new builder-style object to manufacture [`ImportHypervisorConfigurationInput`](crate::input::ImportHypervisorConfigurationInput).
    pub fn builder() -> crate::input::import_hypervisor_configuration_input::Builder {
        crate::input::import_hypervisor_configuration_input::Builder::default()
    }
    /// Creates a new `ImportHypervisorConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportHypervisorConfiguration {
    type Output = std::result::Result<
        crate::output::ImportHypervisorConfigurationOutput,
        crate::error::ImportHypervisorConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_import_hypervisor_configuration_error(response)
        } else {
            crate::operation_deser::parse_import_hypervisor_configuration_response(response)
        }
    }
}

/// Operation shape for `ListGateways`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_gateways`](crate::client::Client::list_gateways).
///
/// See [`crate::client::fluent_builders::ListGateways`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListGateways {
    _private: (),
}
impl ListGateways {
    /// Creates a new builder-style object to manufacture [`ListGatewaysInput`](crate::input::ListGatewaysInput).
    pub fn builder() -> crate::input::list_gateways_input::Builder {
        crate::input::list_gateways_input::Builder::default()
    }
    /// Creates a new `ListGateways` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGateways {
    type Output =
        std::result::Result<crate::output::ListGatewaysOutput, crate::error::ListGatewaysError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_gateways_error(response)
        } else {
            crate::operation_deser::parse_list_gateways_response(response)
        }
    }
}

/// Operation shape for `ListHypervisors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_hypervisors`](crate::client::Client::list_hypervisors).
///
/// See [`crate::client::fluent_builders::ListHypervisors`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListHypervisors {
    _private: (),
}
impl ListHypervisors {
    /// Creates a new builder-style object to manufacture [`ListHypervisorsInput`](crate::input::ListHypervisorsInput).
    pub fn builder() -> crate::input::list_hypervisors_input::Builder {
        crate::input::list_hypervisors_input::Builder::default()
    }
    /// Creates a new `ListHypervisors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListHypervisors {
    type Output = std::result::Result<
        crate::output::ListHypervisorsOutput,
        crate::error::ListHypervisorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_hypervisors_error(response)
        } else {
            crate::operation_deser::parse_list_hypervisors_response(response)
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

/// Operation shape for `ListVirtualMachines`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_virtual_machines`](crate::client::Client::list_virtual_machines).
///
/// See [`crate::client::fluent_builders::ListVirtualMachines`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListVirtualMachines {
    _private: (),
}
impl ListVirtualMachines {
    /// Creates a new builder-style object to manufacture [`ListVirtualMachinesInput`](crate::input::ListVirtualMachinesInput).
    pub fn builder() -> crate::input::list_virtual_machines_input::Builder {
        crate::input::list_virtual_machines_input::Builder::default()
    }
    /// Creates a new `ListVirtualMachines` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListVirtualMachines {
    type Output = std::result::Result<
        crate::output::ListVirtualMachinesOutput,
        crate::error::ListVirtualMachinesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_virtual_machines_error(response)
        } else {
            crate::operation_deser::parse_list_virtual_machines_response(response)
        }
    }
}

/// Operation shape for `PutBandwidthRateLimitSchedule`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_bandwidth_rate_limit_schedule`](crate::client::Client::put_bandwidth_rate_limit_schedule).
///
/// See [`crate::client::fluent_builders::PutBandwidthRateLimitSchedule`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutBandwidthRateLimitSchedule {
    _private: (),
}
impl PutBandwidthRateLimitSchedule {
    /// Creates a new builder-style object to manufacture [`PutBandwidthRateLimitScheduleInput`](crate::input::PutBandwidthRateLimitScheduleInput).
    pub fn builder() -> crate::input::put_bandwidth_rate_limit_schedule_input::Builder {
        crate::input::put_bandwidth_rate_limit_schedule_input::Builder::default()
    }
    /// Creates a new `PutBandwidthRateLimitSchedule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutBandwidthRateLimitSchedule {
    type Output = std::result::Result<
        crate::output::PutBandwidthRateLimitScheduleOutput,
        crate::error::PutBandwidthRateLimitScheduleError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_bandwidth_rate_limit_schedule_error(response)
        } else {
            crate::operation_deser::parse_put_bandwidth_rate_limit_schedule_response(response)
        }
    }
}

/// Operation shape for `PutHypervisorPropertyMappings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_hypervisor_property_mappings`](crate::client::Client::put_hypervisor_property_mappings).
///
/// See [`crate::client::fluent_builders::PutHypervisorPropertyMappings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutHypervisorPropertyMappings {
    _private: (),
}
impl PutHypervisorPropertyMappings {
    /// Creates a new builder-style object to manufacture [`PutHypervisorPropertyMappingsInput`](crate::input::PutHypervisorPropertyMappingsInput).
    pub fn builder() -> crate::input::put_hypervisor_property_mappings_input::Builder {
        crate::input::put_hypervisor_property_mappings_input::Builder::default()
    }
    /// Creates a new `PutHypervisorPropertyMappings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutHypervisorPropertyMappings {
    type Output = std::result::Result<
        crate::output::PutHypervisorPropertyMappingsOutput,
        crate::error::PutHypervisorPropertyMappingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_hypervisor_property_mappings_error(response)
        } else {
            crate::operation_deser::parse_put_hypervisor_property_mappings_response(response)
        }
    }
}

/// Operation shape for `PutMaintenanceStartTime`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_maintenance_start_time`](crate::client::Client::put_maintenance_start_time).
///
/// See [`crate::client::fluent_builders::PutMaintenanceStartTime`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutMaintenanceStartTime {
    _private: (),
}
impl PutMaintenanceStartTime {
    /// Creates a new builder-style object to manufacture [`PutMaintenanceStartTimeInput`](crate::input::PutMaintenanceStartTimeInput).
    pub fn builder() -> crate::input::put_maintenance_start_time_input::Builder {
        crate::input::put_maintenance_start_time_input::Builder::default()
    }
    /// Creates a new `PutMaintenanceStartTime` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutMaintenanceStartTime {
    type Output = std::result::Result<
        crate::output::PutMaintenanceStartTimeOutput,
        crate::error::PutMaintenanceStartTimeError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_maintenance_start_time_error(response)
        } else {
            crate::operation_deser::parse_put_maintenance_start_time_response(response)
        }
    }
}

/// Operation shape for `StartVirtualMachinesMetadataSync`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_virtual_machines_metadata_sync`](crate::client::Client::start_virtual_machines_metadata_sync).
///
/// See [`crate::client::fluent_builders::StartVirtualMachinesMetadataSync`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartVirtualMachinesMetadataSync {
    _private: (),
}
impl StartVirtualMachinesMetadataSync {
    /// Creates a new builder-style object to manufacture [`StartVirtualMachinesMetadataSyncInput`](crate::input::StartVirtualMachinesMetadataSyncInput).
    pub fn builder() -> crate::input::start_virtual_machines_metadata_sync_input::Builder {
        crate::input::start_virtual_machines_metadata_sync_input::Builder::default()
    }
    /// Creates a new `StartVirtualMachinesMetadataSync` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartVirtualMachinesMetadataSync {
    type Output = std::result::Result<
        crate::output::StartVirtualMachinesMetadataSyncOutput,
        crate::error::StartVirtualMachinesMetadataSyncError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_virtual_machines_metadata_sync_error(response)
        } else {
            crate::operation_deser::parse_start_virtual_machines_metadata_sync_response(response)
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

/// Operation shape for `TestHypervisorConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`test_hypervisor_configuration`](crate::client::Client::test_hypervisor_configuration).
///
/// See [`crate::client::fluent_builders::TestHypervisorConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TestHypervisorConfiguration {
    _private: (),
}
impl TestHypervisorConfiguration {
    /// Creates a new builder-style object to manufacture [`TestHypervisorConfigurationInput`](crate::input::TestHypervisorConfigurationInput).
    pub fn builder() -> crate::input::test_hypervisor_configuration_input::Builder {
        crate::input::test_hypervisor_configuration_input::Builder::default()
    }
    /// Creates a new `TestHypervisorConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TestHypervisorConfiguration {
    type Output = std::result::Result<
        crate::output::TestHypervisorConfigurationOutput,
        crate::error::TestHypervisorConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_test_hypervisor_configuration_error(response)
        } else {
            crate::operation_deser::parse_test_hypervisor_configuration_response(response)
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

/// Operation shape for `UpdateGatewayInformation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_gateway_information`](crate::client::Client::update_gateway_information).
///
/// See [`crate::client::fluent_builders::UpdateGatewayInformation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateGatewayInformation {
    _private: (),
}
impl UpdateGatewayInformation {
    /// Creates a new builder-style object to manufacture [`UpdateGatewayInformationInput`](crate::input::UpdateGatewayInformationInput).
    pub fn builder() -> crate::input::update_gateway_information_input::Builder {
        crate::input::update_gateway_information_input::Builder::default()
    }
    /// Creates a new `UpdateGatewayInformation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateGatewayInformation {
    type Output = std::result::Result<
        crate::output::UpdateGatewayInformationOutput,
        crate::error::UpdateGatewayInformationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_gateway_information_error(response)
        } else {
            crate::operation_deser::parse_update_gateway_information_response(response)
        }
    }
}

/// Operation shape for `UpdateGatewaySoftwareNow`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_gateway_software_now`](crate::client::Client::update_gateway_software_now).
///
/// See [`crate::client::fluent_builders::UpdateGatewaySoftwareNow`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateGatewaySoftwareNow {
    _private: (),
}
impl UpdateGatewaySoftwareNow {
    /// Creates a new builder-style object to manufacture [`UpdateGatewaySoftwareNowInput`](crate::input::UpdateGatewaySoftwareNowInput).
    pub fn builder() -> crate::input::update_gateway_software_now_input::Builder {
        crate::input::update_gateway_software_now_input::Builder::default()
    }
    /// Creates a new `UpdateGatewaySoftwareNow` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateGatewaySoftwareNow {
    type Output = std::result::Result<
        crate::output::UpdateGatewaySoftwareNowOutput,
        crate::error::UpdateGatewaySoftwareNowError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_gateway_software_now_error(response)
        } else {
            crate::operation_deser::parse_update_gateway_software_now_response(response)
        }
    }
}

/// Operation shape for `UpdateHypervisor`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_hypervisor`](crate::client::Client::update_hypervisor).
///
/// See [`crate::client::fluent_builders::UpdateHypervisor`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateHypervisor {
    _private: (),
}
impl UpdateHypervisor {
    /// Creates a new builder-style object to manufacture [`UpdateHypervisorInput`](crate::input::UpdateHypervisorInput).
    pub fn builder() -> crate::input::update_hypervisor_input::Builder {
        crate::input::update_hypervisor_input::Builder::default()
    }
    /// Creates a new `UpdateHypervisor` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateHypervisor {
    type Output = std::result::Result<
        crate::output::UpdateHypervisorOutput,
        crate::error::UpdateHypervisorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_hypervisor_error(response)
        } else {
            crate::operation_deser::parse_update_hypervisor_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
