// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRadius`](crate::operation::update_radius::builders::UpdateRadiusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::update_radius::builders::UpdateRadiusFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::update_radius::builders::UpdateRadiusFluentBuilder::set_directory_id):<br>required: **true**<br><p>The identifier of the directory for which to update the RADIUS server information.</p><br>
    ///   - [`radius_settings(RadiusSettings)`](crate::operation::update_radius::builders::UpdateRadiusFluentBuilder::radius_settings) / [`set_radius_settings(Option<RadiusSettings>)`](crate::operation::update_radius::builders::UpdateRadiusFluentBuilder::set_radius_settings):<br>required: **true**<br><p>A <code>RadiusSettings</code> object that contains information about the RADIUS server.</p><br>
    /// - On success, responds with [`UpdateRadiusOutput`](crate::operation::update_radius::UpdateRadiusOutput)
    /// - On failure, responds with [`SdkError<UpdateRadiusError>`](crate::operation::update_radius::UpdateRadiusError)
    pub fn update_radius(&self) -> crate::operation::update_radius::builders::UpdateRadiusFluentBuilder {
        crate::operation::update_radius::builders::UpdateRadiusFluentBuilder::new(self.handle.clone())
    }
}
