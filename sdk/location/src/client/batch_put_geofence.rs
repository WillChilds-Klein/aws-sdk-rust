// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchPutGeofence`](crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`collection_name(impl Into<String>)`](crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder::collection_name) / [`set_collection_name(Option<String>)`](crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder::set_collection_name):<br>required: **true**<br><p>The geofence collection storing the geofences.</p><br>
    ///   - [`entries(BatchPutGeofenceRequestEntry)`](crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder::entries) / [`set_entries(Option<Vec::<BatchPutGeofenceRequestEntry>>)`](crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder::set_entries):<br>required: **true**<br><p>The batch of geofences to be stored in a geofence collection.</p><br>
    /// - On success, responds with [`BatchPutGeofenceOutput`](crate::operation::batch_put_geofence::BatchPutGeofenceOutput) with field(s):
    ///   - [`successes(Vec::<BatchPutGeofenceSuccess>)`](crate::operation::batch_put_geofence::BatchPutGeofenceOutput::successes): <p>Contains each geofence that was successfully stored in a geofence collection.</p>
    ///   - [`errors(Vec::<BatchPutGeofenceError>)`](crate::operation::batch_put_geofence::BatchPutGeofenceOutput::errors): <p>Contains additional error details for each geofence that failed to be stored in a geofence collection.</p>
    /// - On failure, responds with [`SdkError<BatchPutGeofenceError>`](crate::operation::batch_put_geofence::BatchPutGeofenceError)
    pub fn batch_put_geofence(&self) -> crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder {
        crate::operation::batch_put_geofence::builders::BatchPutGeofenceFluentBuilder::new(self.handle.clone())
    }
}
