// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDevices`](crate::operation::list_devices::builders::ListDevicesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::set_next_token):<br>required: **false**<br><p>Specify the pagination token from a previous request to retrieve the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of devices to return in one page of results.</p><br>
    ///   - [`sort_by(ListDevicesSortBy)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::sort_by) / [`set_sort_by(Option<ListDevicesSortBy>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::set_sort_by):<br>required: **false**<br><p>The target column to be sorted on. Default column sort is CREATED_TIME.</p><br>
    ///   - [`sort_order(SortOrder)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::set_sort_order):<br>required: **false**<br><p>The sorting order for the returned list. SortOrder is DESCENDING by default based on CREATED_TIME. Otherwise, SortOrder is ASCENDING.</p><br>
    ///   - [`name_filter(impl Into<String>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::name_filter) / [`set_name_filter(Option<String>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::set_name_filter):<br>required: **false**<br><p>Filter based on device's name. Prefixes supported.</p><br>
    ///   - [`device_aggregated_status_filter(DeviceAggregatedStatus)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::device_aggregated_status_filter) / [`set_device_aggregated_status_filter(Option<DeviceAggregatedStatus>)`](crate::operation::list_devices::builders::ListDevicesFluentBuilder::set_device_aggregated_status_filter):<br>required: **false**<br><p>Filter based on a device's status.</p><br>
    /// - On success, responds with [`ListDevicesOutput`](crate::operation::list_devices::ListDevicesOutput) with field(s):
    ///   - [`devices(Vec::<Device>)`](crate::operation::list_devices::ListDevicesOutput::devices): <p>A list of devices.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_devices::ListDevicesOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListDevicesError>`](crate::operation::list_devices::ListDevicesError)
    pub fn list_devices(&self) -> crate::operation::list_devices::builders::ListDevicesFluentBuilder {
        crate::operation::list_devices::builders::ListDevicesFluentBuilder::new(self.handle.clone())
    }
}
