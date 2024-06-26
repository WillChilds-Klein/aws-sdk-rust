// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSnapshotCopyConfiguration`](crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`snapshot_copy_configuration_id(impl Into<String>)`](crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder::snapshot_copy_configuration_id) / [`set_snapshot_copy_configuration_id(Option<String>)`](crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder::set_snapshot_copy_configuration_id):<br>required: **true**<br><p>The ID of the snapshot copy configuration to update.</p><br>
    ///   - [`snapshot_retention_period(i32)`](crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder::snapshot_retention_period) / [`set_snapshot_retention_period(Option<i32>)`](crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder::set_snapshot_retention_period):<br>required: **false**<br><p>The new retention period of how long to keep a snapshot in the destination Amazon Web Services Region.</p><br>
    /// - On success, responds with [`UpdateSnapshotCopyConfigurationOutput`](crate::operation::update_snapshot_copy_configuration::UpdateSnapshotCopyConfigurationOutput) with field(s):
    ///   - [`snapshot_copy_configuration(Option<SnapshotCopyConfiguration>)`](crate::operation::update_snapshot_copy_configuration::UpdateSnapshotCopyConfigurationOutput::snapshot_copy_configuration): <p>The updated snapshot copy configuration object.</p>
    /// - On failure, responds with [`SdkError<UpdateSnapshotCopyConfigurationError>`](crate::operation::update_snapshot_copy_configuration::UpdateSnapshotCopyConfigurationError)
    pub fn update_snapshot_copy_configuration(
        &self,
    ) -> crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder {
        crate::operation::update_snapshot_copy_configuration::builders::UpdateSnapshotCopyConfigurationFluentBuilder::new(self.handle.clone())
    }
}
