// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The information for public switched telephone network (PSTN) conferencing.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PstnDialIn {
    /// <p>The zip code.</p>
    pub country_code: ::std::string::String,
    /// <p>The phone number to call to join the conference.</p>
    pub phone_number: ::std::string::String,
    /// <p>The delay duration before Alexa enters the conference ID with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub one_click_id_delay: ::std::string::String,
    /// <p>The delay duration before Alexa enters the conference pin with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub one_click_pin_delay: ::std::string::String,
}
impl PstnDialIn {
    /// <p>The zip code.</p>
    pub fn country_code(&self) -> &str {
        use std::ops::Deref;
        self.country_code.deref()
    }
    /// <p>The phone number to call to join the conference.</p>
    pub fn phone_number(&self) -> &str {
        use std::ops::Deref;
        self.phone_number.deref()
    }
    /// <p>The delay duration before Alexa enters the conference ID with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub fn one_click_id_delay(&self) -> &str {
        use std::ops::Deref;
        self.one_click_id_delay.deref()
    }
    /// <p>The delay duration before Alexa enters the conference pin with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub fn one_click_pin_delay(&self) -> &str {
        use std::ops::Deref;
        self.one_click_pin_delay.deref()
    }
}
impl PstnDialIn {
    /// Creates a new builder-style object to manufacture [`PstnDialIn`](crate::types::PstnDialIn).
    pub fn builder() -> crate::types::builders::PstnDialInBuilder {
        crate::types::builders::PstnDialInBuilder::default()
    }
}

/// A builder for [`PstnDialIn`](crate::types::PstnDialIn).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PstnDialInBuilder {
    pub(crate) country_code: ::std::option::Option<::std::string::String>,
    pub(crate) phone_number: ::std::option::Option<::std::string::String>,
    pub(crate) one_click_id_delay: ::std::option::Option<::std::string::String>,
    pub(crate) one_click_pin_delay: ::std::option::Option<::std::string::String>,
}
impl PstnDialInBuilder {
    /// <p>The zip code.</p>
    /// This field is required.
    pub fn country_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.country_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The zip code.</p>
    pub fn set_country_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.country_code = input;
        self
    }
    /// <p>The zip code.</p>
    pub fn get_country_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.country_code
    }
    /// <p>The phone number to call to join the conference.</p>
    /// This field is required.
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.phone_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The phone number to call to join the conference.</p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.phone_number = input;
        self
    }
    /// <p>The phone number to call to join the conference.</p>
    pub fn get_phone_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.phone_number
    }
    /// <p>The delay duration before Alexa enters the conference ID with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    /// This field is required.
    pub fn one_click_id_delay(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.one_click_id_delay = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The delay duration before Alexa enters the conference ID with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub fn set_one_click_id_delay(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.one_click_id_delay = input;
        self
    }
    /// <p>The delay duration before Alexa enters the conference ID with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub fn get_one_click_id_delay(&self) -> &::std::option::Option<::std::string::String> {
        &self.one_click_id_delay
    }
    /// <p>The delay duration before Alexa enters the conference pin with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    /// This field is required.
    pub fn one_click_pin_delay(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.one_click_pin_delay = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The delay duration before Alexa enters the conference pin with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub fn set_one_click_pin_delay(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.one_click_pin_delay = input;
        self
    }
    /// <p>The delay duration before Alexa enters the conference pin with dual-tone multi-frequency (DTMF). Each number on the dial pad corresponds to a DTMF tone, which is how we send data over the telephone network.</p>
    pub fn get_one_click_pin_delay(&self) -> &::std::option::Option<::std::string::String> {
        &self.one_click_pin_delay
    }
    /// Consumes the builder and constructs a [`PstnDialIn`](crate::types::PstnDialIn).
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](crate::types::builders::PstnDialInBuilder::country_code)
    /// - [`phone_number`](crate::types::builders::PstnDialInBuilder::phone_number)
    /// - [`one_click_id_delay`](crate::types::builders::PstnDialInBuilder::one_click_id_delay)
    /// - [`one_click_pin_delay`](crate::types::builders::PstnDialInBuilder::one_click_pin_delay)
    pub fn build(self) -> ::std::result::Result<crate::types::PstnDialIn, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::PstnDialIn {
            country_code: self.country_code.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "country_code",
                    "country_code was not specified but it is required when building PstnDialIn",
                )
            })?,
            phone_number: self.phone_number.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "phone_number",
                    "phone_number was not specified but it is required when building PstnDialIn",
                )
            })?,
            one_click_id_delay: self.one_click_id_delay.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "one_click_id_delay",
                    "one_click_id_delay was not specified but it is required when building PstnDialIn",
                )
            })?,
            one_click_pin_delay: self.one_click_pin_delay.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "one_click_pin_delay",
                    "one_click_pin_delay was not specified but it is required when building PstnDialIn",
                )
            })?,
        })
    }
}
