// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DetectProfileObjectType`](crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`objects(impl Into<String>)`](crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder::objects) / [`set_objects(Option<Vec::<String>>)`](crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder::set_objects):<br>required: **true**<br><p>A string that is serialized from a JSON object.</p><br>
    ///   - [`domain_name(impl Into<String>)`](crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder::set_domain_name):<br>required: **true**<br><p>The unique name of the domain.</p><br>
    /// - On success, responds with [`DetectProfileObjectTypeOutput`](crate::operation::detect_profile_object_type::DetectProfileObjectTypeOutput) with field(s):
    ///   - [`detected_profile_object_types(Option<Vec::<DetectedProfileObjectType>>)`](crate::operation::detect_profile_object_type::DetectProfileObjectTypeOutput::detected_profile_object_types): <p>Detected <code>ProfileObjectType</code> mappings from given objects. A maximum of one mapping is supported.</p>
    /// - On failure, responds with [`SdkError<DetectProfileObjectTypeError>`](crate::operation::detect_profile_object_type::DetectProfileObjectTypeError)
    pub fn detect_profile_object_type(&self) -> crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder {
        crate::operation::detect_profile_object_type::builders::DetectProfileObjectTypeFluentBuilder::new(self.handle.clone())
    }
}
