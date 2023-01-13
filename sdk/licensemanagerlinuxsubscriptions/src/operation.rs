// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetServiceSettings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_service_settings`](crate::client::Client::get_service_settings).
///
/// See [`crate::client::fluent_builders::GetServiceSettings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetServiceSettings {
    _private: (),
}
impl GetServiceSettings {
    /// Creates a new builder-style object to manufacture [`GetServiceSettingsInput`](crate::input::GetServiceSettingsInput).
    pub fn builder() -> crate::input::get_service_settings_input::Builder {
        crate::input::get_service_settings_input::Builder::default()
    }
    /// Creates a new `GetServiceSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServiceSettings {
    type Output = std::result::Result<
        crate::output::GetServiceSettingsOutput,
        crate::error::GetServiceSettingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_service_settings_error(response)
        } else {
            crate::operation_deser::parse_get_service_settings_response(response)
        }
    }
}

/// Operation shape for `ListLinuxSubscriptionInstances`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_linux_subscription_instances`](crate::client::Client::list_linux_subscription_instances).
///
/// See [`crate::client::fluent_builders::ListLinuxSubscriptionInstances`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLinuxSubscriptionInstances {
    _private: (),
}
impl ListLinuxSubscriptionInstances {
    /// Creates a new builder-style object to manufacture [`ListLinuxSubscriptionInstancesInput`](crate::input::ListLinuxSubscriptionInstancesInput).
    pub fn builder() -> crate::input::list_linux_subscription_instances_input::Builder {
        crate::input::list_linux_subscription_instances_input::Builder::default()
    }
    /// Creates a new `ListLinuxSubscriptionInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLinuxSubscriptionInstances {
    type Output = std::result::Result<
        crate::output::ListLinuxSubscriptionInstancesOutput,
        crate::error::ListLinuxSubscriptionInstancesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_linux_subscription_instances_error(response)
        } else {
            crate::operation_deser::parse_list_linux_subscription_instances_response(response)
        }
    }
}

/// Operation shape for `ListLinuxSubscriptions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_linux_subscriptions`](crate::client::Client::list_linux_subscriptions).
///
/// See [`crate::client::fluent_builders::ListLinuxSubscriptions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLinuxSubscriptions {
    _private: (),
}
impl ListLinuxSubscriptions {
    /// Creates a new builder-style object to manufacture [`ListLinuxSubscriptionsInput`](crate::input::ListLinuxSubscriptionsInput).
    pub fn builder() -> crate::input::list_linux_subscriptions_input::Builder {
        crate::input::list_linux_subscriptions_input::Builder::default()
    }
    /// Creates a new `ListLinuxSubscriptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLinuxSubscriptions {
    type Output = std::result::Result<
        crate::output::ListLinuxSubscriptionsOutput,
        crate::error::ListLinuxSubscriptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_linux_subscriptions_error(response)
        } else {
            crate::operation_deser::parse_list_linux_subscriptions_response(response)
        }
    }
}

/// Operation shape for `UpdateServiceSettings`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_service_settings`](crate::client::Client::update_service_settings).
///
/// See [`crate::client::fluent_builders::UpdateServiceSettings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateServiceSettings {
    _private: (),
}
impl UpdateServiceSettings {
    /// Creates a new builder-style object to manufacture [`UpdateServiceSettingsInput`](crate::input::UpdateServiceSettingsInput).
    pub fn builder() -> crate::input::update_service_settings_input::Builder {
        crate::input::update_service_settings_input::Builder::default()
    }
    /// Creates a new `UpdateServiceSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateServiceSettings {
    type Output = std::result::Result<
        crate::output::UpdateServiceSettingsOutput,
        crate::error::UpdateServiceSettingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_service_settings_error(response)
        } else {
            crate::operation_deser::parse_update_service_settings_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
