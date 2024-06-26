// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Creates settings for the require check in feature that are applied to a room profile. Require check in allows a meeting room’s Alexa or AVS device to prompt the user to check in; otherwise, the room will be released.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateRequireCheckIn {
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into.</p>
    pub release_after_minutes: i32,
    /// <p>Whether require check in is enabled or not.</p>
    pub enabled: bool,
}
impl CreateRequireCheckIn {
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into.</p>
    pub fn release_after_minutes(&self) -> i32 {
        self.release_after_minutes
    }
    /// <p>Whether require check in is enabled or not.</p>
    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
impl CreateRequireCheckIn {
    /// Creates a new builder-style object to manufacture [`CreateRequireCheckIn`](crate::types::CreateRequireCheckIn).
    pub fn builder() -> crate::types::builders::CreateRequireCheckInBuilder {
        crate::types::builders::CreateRequireCheckInBuilder::default()
    }
}

/// A builder for [`CreateRequireCheckIn`](crate::types::CreateRequireCheckIn).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateRequireCheckInBuilder {
    pub(crate) release_after_minutes: ::std::option::Option<i32>,
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl CreateRequireCheckInBuilder {
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into.</p>
    /// This field is required.
    pub fn release_after_minutes(mut self, input: i32) -> Self {
        self.release_after_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into.</p>
    pub fn set_release_after_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.release_after_minutes = input;
        self
    }
    /// <p>Duration between 5 and 20 minutes to determine when to release the room if it's not checked into.</p>
    pub fn get_release_after_minutes(&self) -> &::std::option::Option<i32> {
        &self.release_after_minutes
    }
    /// <p>Whether require check in is enabled or not.</p>
    /// This field is required.
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether require check in is enabled or not.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Whether require check in is enabled or not.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// Consumes the builder and constructs a [`CreateRequireCheckIn`](crate::types::CreateRequireCheckIn).
    /// This method will fail if any of the following fields are not set:
    /// - [`release_after_minutes`](crate::types::builders::CreateRequireCheckInBuilder::release_after_minutes)
    /// - [`enabled`](crate::types::builders::CreateRequireCheckInBuilder::enabled)
    pub fn build(self) -> ::std::result::Result<crate::types::CreateRequireCheckIn, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::CreateRequireCheckIn {
            release_after_minutes: self.release_after_minutes.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "release_after_minutes",
                    "release_after_minutes was not specified but it is required when building CreateRequireCheckIn",
                )
            })?,
            enabled: self.enabled.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "enabled",
                    "enabled was not specified but it is required when building CreateRequireCheckIn",
                )
            })?,
        })
    }
}
