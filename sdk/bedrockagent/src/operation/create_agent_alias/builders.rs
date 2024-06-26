// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_agent_alias::_create_agent_alias_output::CreateAgentAliasOutputBuilder;

pub use crate::operation::create_agent_alias::_create_agent_alias_input::CreateAgentAliasInputBuilder;

impl crate::operation::create_agent_alias::builders::CreateAgentAliasInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_agent_alias::CreateAgentAliasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_agent_alias::CreateAgentAliasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_agent_alias();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAgentAlias`.
///
/// <p>Creates an alias of an agent that can be used to deploy the agent.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAgentAliasFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_agent_alias::builders::CreateAgentAliasInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_agent_alias::CreateAgentAliasOutput,
        crate::operation::create_agent_alias::CreateAgentAliasError,
    > for CreateAgentAliasFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_agent_alias::CreateAgentAliasOutput,
            crate::operation::create_agent_alias::CreateAgentAliasError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAgentAliasFluentBuilder {
    /// Creates a new `CreateAgentAlias`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAgentAlias as a reference.
    pub fn as_input(&self) -> &crate::operation::create_agent_alias::builders::CreateAgentAliasInputBuilder {
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
        crate::operation::create_agent_alias::CreateAgentAliasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_agent_alias::CreateAgentAliasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_agent_alias::CreateAgentAlias::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_agent_alias::CreateAgentAlias::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_agent_alias::CreateAgentAliasOutput,
        crate::operation::create_agent_alias::CreateAgentAliasError,
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
    /// <p>The unique identifier of the agent.</p>
    pub fn agent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_id(input.into());
        self
    }
    /// <p>The unique identifier of the agent.</p>
    pub fn set_agent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_id(input);
        self
    }
    /// <p>The unique identifier of the agent.</p>
    pub fn get_agent_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_id()
    }
    /// <p>The name of the alias.</p>
    pub fn agent_alias_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.agent_alias_name(input.into());
        self
    }
    /// <p>The name of the alias.</p>
    pub fn set_agent_alias_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_agent_alias_name(input);
        self
    }
    /// <p>The name of the alias.</p>
    pub fn get_agent_alias_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_agent_alias_name()
    }
    /// <p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>A description of the alias of the agent.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the alias of the agent.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the alias of the agent.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    ///
    /// Appends an item to `routingConfiguration`.
    ///
    /// To override the contents of this collection use [`set_routing_configuration`](Self::set_routing_configuration).
    ///
    /// <p>Contains details about the routing configuration of the alias.</p>
    pub fn routing_configuration(mut self, input: crate::types::AgentAliasRoutingConfigurationListItem) -> Self {
        self.inner = self.inner.routing_configuration(input);
        self
    }
    /// <p>Contains details about the routing configuration of the alias.</p>
    pub fn set_routing_configuration(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AgentAliasRoutingConfigurationListItem>>,
    ) -> Self {
        self.inner = self.inner.set_routing_configuration(input);
        self
    }
    /// <p>Contains details about the routing configuration of the alias.</p>
    pub fn get_routing_configuration(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AgentAliasRoutingConfigurationListItem>> {
        self.inner.get_routing_configuration()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags that you want to attach to the alias of the agent.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Any tags that you want to attach to the alias of the agent.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Any tags that you want to attach to the alias of the agent.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
