// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The shard iterator has expired and can no longer be used to retrieve stream records. A shard iterator expires 15 minutes after it is retrieved using the <code>GetShardIterator</code> action.</p>
    ExpiredIteratorException(crate::error::ExpiredIteratorException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>There is no limit to the number of daily on-demand backups that can be taken. </p>
    /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p>
    /// <p>When you are creating a table with one or more secondary indexes, you can have up to 250 such requests running at a time. However, if the table or index specifications are complex, then DynamoDB might temporarily reduce the number of concurrent operations.</p>
    /// <p>When importing into DynamoDB, up to 50 simultaneous import table operations are allowed per account.</p>
    /// <p>There is a soft account quota of 2,500 tables.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation attempted to read past the oldest stream record in a shard.</p>
    /// <p>In DynamoDB Streams, there is a 24 hour limit on data retention. Stream records whose age exceeds this limit are subject to removal (trimming) from the stream. You might receive a TrimmedDataAccessException if:</p>
    /// <ul>
    /// <li> <p>You request a shard iterator with a sequence number older than the trim point (24 hours).</p> </li>
    /// <li> <p>You obtain a shard iterator, but before you use the iterator in a <code>GetRecords</code> request, a stream record in the shard exceeds the 24 hour period and is trimmed. This causes the iterator to access a record that no longer exists.</p> </li>
    /// </ul>
    TrimmedDataAccessException(crate::error::TrimmedDataAccessException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExpiredIteratorException(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TrimmedDataAccessException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeStreamError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeStreamError> for Error {
    fn from(err: crate::error::DescribeStreamError) -> Self {
        match err.kind {
            crate::error::DescribeStreamErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::DescribeStreamErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeStreamErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRecordsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRecordsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetRecordsError> for Error {
    fn from(err: crate::error::GetRecordsError) -> Self {
        match err.kind {
            crate::error::GetRecordsErrorKind::ExpiredIteratorException(inner) => {
                Error::ExpiredIteratorException(inner)
            }
            crate::error::GetRecordsErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::GetRecordsErrorKind::LimitExceededException(inner) => {
                Error::LimitExceededException(inner)
            }
            crate::error::GetRecordsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetRecordsErrorKind::TrimmedDataAccessException(inner) => {
                Error::TrimmedDataAccessException(inner)
            }
            crate::error::GetRecordsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetShardIteratorError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetShardIteratorError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetShardIteratorError> for Error {
    fn from(err: crate::error::GetShardIteratorError) -> Self {
        match err.kind {
            crate::error::GetShardIteratorErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::GetShardIteratorErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::GetShardIteratorErrorKind::TrimmedDataAccessException(inner) => {
                Error::TrimmedDataAccessException(inner)
            }
            crate::error::GetShardIteratorErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListStreamsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListStreamsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListStreamsError> for Error {
    fn from(err: crate::error::ListStreamsError) -> Self {
        match err.kind {
            crate::error::ListStreamsErrorKind::InternalServerError(inner) => {
                Error::InternalServerError(inner)
            }
            crate::error::ListStreamsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListStreamsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
