// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct StartConversationRequestEventStreamErrorMarshaller;

impl StartConversationRequestEventStreamErrorMarshaller {
    pub fn new() -> Self {
        StartConversationRequestEventStreamErrorMarshaller
    }
}
impl ::aws_smithy_eventstream::frame::MarshallMessage for StartConversationRequestEventStreamErrorMarshaller {
    type Input = crate::types::error::StartConversationRequestEventStreamError;
    fn marshall(
        &self,
        _input: Self::Input,
    ) -> std::result::Result<::aws_smithy_types::event_stream::Message, ::aws_smithy_eventstream::error::Error> {
        let mut headers = Vec::new();
        headers.push(::aws_smithy_types::event_stream::Header::new(
            ":message-type",
            ::aws_smithy_types::event_stream::HeaderValue::String("exception".into()),
        ));
        let payload = Vec::new();
        Ok(::aws_smithy_types::event_stream::Message::new_from_parts(headers, payload))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct StartConversationRequestEventStreamMarshaller;

impl StartConversationRequestEventStreamMarshaller {
    pub fn new() -> Self {
        StartConversationRequestEventStreamMarshaller
    }
}
impl ::aws_smithy_eventstream::frame::MarshallMessage for StartConversationRequestEventStreamMarshaller {
    type Input = crate::types::StartConversationRequestEventStream;
    fn marshall(&self, input: Self::Input) -> std::result::Result<::aws_smithy_types::event_stream::Message, ::aws_smithy_eventstream::error::Error> {
        let mut headers = Vec::new();
        headers.push(::aws_smithy_types::event_stream::Header::new(
            ":message-type",
            ::aws_smithy_types::event_stream::HeaderValue::String("event".into()),
        ));
        let payload = match input {
            Self::Input::ConfigurationEvent(inner) =>  {
                headers.push(::aws_smithy_types::event_stream::Header::new(":event-type", ::aws_smithy_types::event_stream::HeaderValue::String("ConfigurationEvent".into())));
                headers.push(::aws_smithy_types::event_stream::Header::new(":content-type", ::aws_smithy_types::event_stream::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_start_conversation_request_event_stream::ser_configuration_event_payload(&inner)
                                            .map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::AudioInputEvent(inner) =>  {
                headers.push(::aws_smithy_types::event_stream::Header::new(":event-type", ::aws_smithy_types::event_stream::HeaderValue::String("AudioInputEvent".into())));
                headers.push(::aws_smithy_types::event_stream::Header::new(":content-type", ::aws_smithy_types::event_stream::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_start_conversation_request_event_stream::ser_audio_input_event_payload(&inner)
                                            .map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::DtmfInputEvent(inner) =>  {
                headers.push(::aws_smithy_types::event_stream::Header::new(":event-type", ::aws_smithy_types::event_stream::HeaderValue::String("DTMFInputEvent".into())));
                headers.push(::aws_smithy_types::event_stream::Header::new(":content-type", ::aws_smithy_types::event_stream::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_start_conversation_request_event_stream::ser_dtmf_input_event_payload(&inner)
                                            .map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::TextInputEvent(inner) =>  {
                headers.push(::aws_smithy_types::event_stream::Header::new(":event-type", ::aws_smithy_types::event_stream::HeaderValue::String("TextInputEvent".into())));
                headers.push(::aws_smithy_types::event_stream::Header::new(":content-type", ::aws_smithy_types::event_stream::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_start_conversation_request_event_stream::ser_text_input_event_payload(&inner)
                                            .map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::PlaybackCompletionEvent(inner) =>  {
                headers.push(::aws_smithy_types::event_stream::Header::new(":event-type", ::aws_smithy_types::event_stream::HeaderValue::String("PlaybackCompletionEvent".into())));
                headers.push(::aws_smithy_types::event_stream::Header::new(":content-type", ::aws_smithy_types::event_stream::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_start_conversation_request_event_stream::ser_playback_completion_event_payload(&inner)
                                            .map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::DisconnectionEvent(inner) =>  {
                headers.push(::aws_smithy_types::event_stream::Header::new(":event-type", ::aws_smithy_types::event_stream::HeaderValue::String("DisconnectionEvent".into())));
                headers.push(::aws_smithy_types::event_stream::Header::new(":content-type", ::aws_smithy_types::event_stream::HeaderValue::String("application/vnd.amazon.eventstream".into())));
                crate::protocol_serde::shape_start_conversation_request_event_stream::ser_disconnection_event_payload(&inner)
                                            .map_err(|err| ::aws_smithy_eventstream::error::Error::marshalling(format!("{}", err)))?
            }
            Self::Input::Unknown => return Err(
                                            ::aws_smithy_eventstream::error::Error::marshalling("Cannot serialize `StartConversationRequestEventStream::Unknown` for the request. The `Unknown` variant is intended for responses only. It occurs when an outdated client is used after a new enum variant was added on the server side.".to_owned())
                                        )
        }
        ;
        Ok(::aws_smithy_types::event_stream::Message::new_from_parts(headers, payload))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct StartConversationResponseEventStreamUnmarshaller;

impl StartConversationResponseEventStreamUnmarshaller {
    pub fn new() -> Self {
        StartConversationResponseEventStreamUnmarshaller
    }
}
impl ::aws_smithy_eventstream::frame::UnmarshallMessage for StartConversationResponseEventStreamUnmarshaller {
    type Output = crate::types::StartConversationResponseEventStream;
    type Error = crate::types::error::StartConversationResponseEventStreamError;
    fn unmarshall(
        &self,
        message: &::aws_smithy_types::event_stream::Message,
    ) -> std::result::Result<::aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>, ::aws_smithy_eventstream::error::Error>
    {
        let response_headers = ::aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "PlaybackInterruptionEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_playback_interruption_event::de_playback_interruption_event_payload(&message.payload()[..])
                            .map_err(|err| {
                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                    "failed to unmarshall PlaybackInterruptionEvent: {}",
                                    err
                                ))
                            })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::StartConversationResponseEventStream::PlaybackInterruptionEvent(parsed),
                    ))
                }
                "TranscriptEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_transcript_event::de_transcript_event_payload(&message.payload()[..]).map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall TranscriptEvent: {}", err))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::StartConversationResponseEventStream::TranscriptEvent(parsed),
                    ))
                }
                "IntentResultEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_intent_result_event::de_intent_result_event_payload(&message.payload()[..]).map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall IntentResultEvent: {}", err))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::StartConversationResponseEventStream::IntentResultEvent(parsed),
                    ))
                }
                "TextResponseEvent" => {
                    let parsed =
                        crate::protocol_serde::shape_text_response_event::de_text_response_event_payload(&message.payload()[..]).map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall TextResponseEvent: {}", err))
                        })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::StartConversationResponseEventStream::TextResponseEvent(parsed),
                    ))
                }
                "AudioResponseEvent" => {
                    let parsed = crate::protocol_serde::shape_audio_response_event::de_audio_response_event_payload(&message.payload()[..]).map_err(
                        |err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall AudioResponseEvent: {}", err)),
                    )?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::StartConversationResponseEventStream::AudioResponseEvent(parsed),
                    ))
                }
                "HeartbeatEvent" => {
                    let parsed = crate::protocol_serde::shape_heartbeat_event::de_heartbeat_event_payload(&message.payload()[..]).map_err(|err| {
                        ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall HeartbeatEvent: {}", err))
                    })?;
                    Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::types::StartConversationResponseEventStream::HeartbeatEvent(parsed),
                    ))
                }
                _unknown_variant => Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::types::StartConversationResponseEventStream::Unknown,
                )),
            },
            "exception" => {
                let generic = match crate::protocol_serde::parse_event_stream_error_metadata(message.payload()) {
                    Ok(builder) => builder.build(),
                    Err(err) => {
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::unhandled(err),
                        ))
                    }
                };
                match response_headers.smithy_type.as_str() {
                    "AccessDeniedException" => {
                        let mut builder = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(
                            &message.payload()[..],
                            builder,
                        )
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall AccessDeniedException: {}", err))
                        })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::AccessDeniedException(
                                crate::serde_util::access_denied_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "ResourceNotFoundException" => {
                        let mut builder = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(
                            &message.payload()[..],
                            builder,
                        )
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ResourceNotFoundException: {}", err))
                        })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::ResourceNotFoundException(
                                crate::serde_util::resource_not_found_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "ValidationException" => {
                        let mut builder = crate::types::error::builders::ValidationExceptionBuilder::default();
                        builder =
                            crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(&message.payload()[..], builder)
                                .map_err(|err| {
                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                        "failed to unmarshall ValidationException: {}",
                                        err
                                    ))
                                })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::ValidationException(
                                crate::serde_util::validation_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "ThrottlingException" => {
                        let mut builder = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                        builder =
                            crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(&message.payload()[..], builder)
                                .map_err(|err| {
                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                        "failed to unmarshall ThrottlingException: {}",
                                        err
                                    ))
                                })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::ThrottlingException(
                                crate::serde_util::throttling_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "InternalServerException" => {
                        let mut builder = crate::types::error::builders::InternalServerExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(
                            &message.payload()[..],
                            builder,
                        )
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InternalServerException: {}", err))
                        })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::InternalServerException(
                                crate::serde_util::internal_server_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "ConflictException" => {
                        let mut builder = crate::types::error::builders::ConflictExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(&message.payload()[..], builder)
                            .map_err(|err| {
                                ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ConflictException: {}", err))
                            })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::ConflictException(
                                crate::serde_util::conflict_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "DependencyFailedException" => {
                        let mut builder = crate::types::error::builders::DependencyFailedExceptionBuilder::default();
                        builder = crate::protocol_serde::shape_dependency_failed_exception::de_dependency_failed_exception_json_err(
                            &message.payload()[..],
                            builder,
                        )
                        .map_err(|err| {
                            ::aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall DependencyFailedException: {}", err))
                        })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::DependencyFailedException(
                                crate::serde_util::dependency_failed_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    "BadGatewayException" => {
                        let mut builder = crate::types::error::builders::BadGatewayExceptionBuilder::default();
                        builder =
                            crate::protocol_serde::shape_bad_gateway_exception::de_bad_gateway_exception_json_err(&message.payload()[..], builder)
                                .map_err(|err| {
                                    ::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                                        "failed to unmarshall BadGatewayException: {}",
                                        err
                                    ))
                                })?;
                        builder.set_meta(Some(generic));
                        return Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::types::error::StartConversationResponseEventStreamError::BadGatewayException(
                                crate::serde_util::bad_gateway_exception_correct_errors(builder)
                                    .build()
                                    .map_err(|err| ::aws_smithy_eventstream::error::Error::unmarshalling(format!("{}", err)))?,
                            ),
                        ));
                    }
                    _ => {}
                }
                Ok(::aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::types::error::StartConversationResponseEventStreamError::generic(generic),
                ))
            }
            value => {
                return Err(::aws_smithy_eventstream::error::Error::unmarshalling(format!(
                    "unrecognized :message-type: {}",
                    value
                )));
            }
        }
    }
}
