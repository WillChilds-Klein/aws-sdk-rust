// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_devices::_deregister_devices_output::DeregisterDevicesOutputBuilder;

pub use crate::operation::deregister_devices::_deregister_devices_input::DeregisterDevicesInputBuilder;

impl crate::operation::deregister_devices::builders::DeregisterDevicesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deregister_devices::DeregisterDevicesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_devices::DeregisterDevicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deregister_devices();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeregisterDevices`.
///
/// <p>Deregisters the specified devices. After you deregister a device, you will need to re-register the devices.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterDevicesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deregister_devices::builders::DeregisterDevicesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deregister_devices::DeregisterDevicesOutput,
        crate::operation::deregister_devices::DeregisterDevicesError,
    > for DeregisterDevicesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deregister_devices::DeregisterDevicesOutput,
            crate::operation::deregister_devices::DeregisterDevicesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeregisterDevicesFluentBuilder {
    /// Creates a new `DeregisterDevices`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeregisterDevices as a reference.
    pub fn as_input(&self) -> &crate::operation::deregister_devices::builders::DeregisterDevicesInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::deregister_devices::DeregisterDevicesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_devices::DeregisterDevicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deregister_devices::DeregisterDevices::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deregister_devices::DeregisterDevices::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deregister_devices::DeregisterDevicesOutput,
        crate::operation::deregister_devices::DeregisterDevicesError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn device_fleet_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_fleet_name(input.into());
        self
    }
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn set_device_fleet_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_fleet_name(input);
        self
    }
    /// <p>The name of the fleet the devices belong to.</p>
    pub fn get_device_fleet_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_fleet_name()
    }
    ///
    /// Appends an item to `DeviceNames`.
    ///
    /// To override the contents of this collection use [`set_device_names`](Self::set_device_names).
    ///
    /// <p>The unique IDs of the devices.</p>
    pub fn device_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_names(input.into());
        self
    }
    /// <p>The unique IDs of the devices.</p>
    pub fn set_device_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_device_names(input);
        self
    }
    /// <p>The unique IDs of the devices.</p>
    pub fn get_device_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_device_names()
    }
}
