// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurchaseOffering`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`count(i32)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::count) / [`set_count(Option<i32>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_count):<br>required: **true**<br>Number of resources<br>
    ///   - [`name(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_name):<br>required: **false**<br>Name for the new reservation<br>
    ///   - [`offering_id(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::offering_id) / [`set_offering_id(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_offering_id):<br>required: **true**<br>Offering to purchase, e.g. '87654321'<br>
    ///   - [`renewal_settings(RenewalSettings)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::renewal_settings) / [`set_renewal_settings(Option<RenewalSettings>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_renewal_settings):<br>required: **false**<br>Renewal settings for the reservation<br>
    ///   - [`request_id(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::request_id) / [`set_request_id(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_request_id):<br>required: **false**<br>Unique request ID to be specified. This is needed to prevent retries from creating multiple resources.<br>
    ///   - [`start(impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::start) / [`set_start(Option<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_start):<br>required: **false**<br>Requested reservation start time (UTC) in ISO-8601 format. The specified time must be between the first day of the current month and one year from now. If no value is given, the default is now.<br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::set_tags):<br>required: **false**<br>A collection of key-value pairs<br>
    /// - On success, responds with [`PurchaseOfferingOutput`](crate::operation::purchase_offering::PurchaseOfferingOutput) with field(s):
    ///   - [`reservation(Option<Reservation>)`](crate::operation::purchase_offering::PurchaseOfferingOutput::reservation): Reserved resources available to use
    /// - On failure, responds with [`SdkError<PurchaseOfferingError>`](crate::operation::purchase_offering::PurchaseOfferingError)
    pub fn purchase_offering(&self) -> crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder {
        crate::operation::purchase_offering::builders::PurchaseOfferingFluentBuilder::new(self.handle.clone())
    }
}
