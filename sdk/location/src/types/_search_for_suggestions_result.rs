// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains a place suggestion resulting from a place suggestion query that is run on a place index resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchForSuggestionsResult {
    /// <p>The text of the place suggestion, typically formatted as an address string.</p>
    pub text: ::std::string::String,
    /// <p>The unique identifier of the Place. You can use this with the <code>GetPlace</code> operation to find the place again later, or to get full information for the Place.</p>
    /// <p>The <code>GetPlace</code> request must use the same <code>PlaceIndex</code> resource as the <code>SearchPlaceIndexForSuggestions</code> that generated the Place ID.</p><note>
    /// <p>For <code>SearchPlaceIndexForSuggestions</code> operations, the <code>PlaceId</code> is returned by place indexes that use Esri, Grab, or HERE as data providers.</p>
    /// </note>
    /// <p>While you can use PlaceID in subsequent requests, PlaceID is not intended to be a permanent identifier and the ID can change between consecutive API calls. Please see the following PlaceID behaviour for each data provider:</p>
    /// <ul>
    /// <li>
    /// <p>Esri: Place IDs will change every quarter at a minimum. The typical time period for these changes would be March, June, September, and December. Place IDs might also change between the typical quarterly change but that will be much less frequent.</p></li>
    /// <li>
    /// <p>HERE: We recommend that you cache data for no longer than a week to keep your data data fresh. You can assume that less than 1% ID shifts will release over release which is approximately 1 - 2 times per week.</p></li>
    /// <li>
    /// <p>Grab: Place IDs can expire or become invalid in the following situations.</p>
    /// <ul>
    /// <li>
    /// <p>Data operations: The POI may be removed from Grab POI database by Grab Map Ops based on the ground-truth, such as being closed in the real world, being detected as a duplicate POI, or having incorrect information. Grab will synchronize data to the Waypoint environment on weekly basis.</p></li>
    /// <li>
    /// <p>Interpolated POI: Interpolated POI is a temporary POI generated in real time when serving a request, and it will be marked as derived in the <code>place.result_type</code> field in the response. The information of interpolated POIs will be retained for at least 30 days, which means that within 30 days, you are able to obtain POI details by Place ID from Place Details API. After 30 days, the interpolated POIs(both Place ID and details) may expire and inaccessible from the Places Details API.</p></li>
    /// </ul></li>
    /// </ul>
    pub place_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Location categories that describe the Place.</p>
    /// <p>For more information about using categories, including a list of Amazon Location categories, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/category-filtering.html">Categories and filtering</a>, in the <i>Amazon Location Service Developer Guide</i>.</p>
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Categories from the data provider that describe the Place that are not mapped to any Amazon Location categories.</p>
    pub supplemental_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl SearchForSuggestionsResult {
    /// <p>The text of the place suggestion, typically formatted as an address string.</p>
    pub fn text(&self) -> &str {
        use std::ops::Deref;
        self.text.deref()
    }
    /// <p>The unique identifier of the Place. You can use this with the <code>GetPlace</code> operation to find the place again later, or to get full information for the Place.</p>
    /// <p>The <code>GetPlace</code> request must use the same <code>PlaceIndex</code> resource as the <code>SearchPlaceIndexForSuggestions</code> that generated the Place ID.</p><note>
    /// <p>For <code>SearchPlaceIndexForSuggestions</code> operations, the <code>PlaceId</code> is returned by place indexes that use Esri, Grab, or HERE as data providers.</p>
    /// </note>
    /// <p>While you can use PlaceID in subsequent requests, PlaceID is not intended to be a permanent identifier and the ID can change between consecutive API calls. Please see the following PlaceID behaviour for each data provider:</p>
    /// <ul>
    /// <li>
    /// <p>Esri: Place IDs will change every quarter at a minimum. The typical time period for these changes would be March, June, September, and December. Place IDs might also change between the typical quarterly change but that will be much less frequent.</p></li>
    /// <li>
    /// <p>HERE: We recommend that you cache data for no longer than a week to keep your data data fresh. You can assume that less than 1% ID shifts will release over release which is approximately 1 - 2 times per week.</p></li>
    /// <li>
    /// <p>Grab: Place IDs can expire or become invalid in the following situations.</p>
    /// <ul>
    /// <li>
    /// <p>Data operations: The POI may be removed from Grab POI database by Grab Map Ops based on the ground-truth, such as being closed in the real world, being detected as a duplicate POI, or having incorrect information. Grab will synchronize data to the Waypoint environment on weekly basis.</p></li>
    /// <li>
    /// <p>Interpolated POI: Interpolated POI is a temporary POI generated in real time when serving a request, and it will be marked as derived in the <code>place.result_type</code> field in the response. The information of interpolated POIs will be retained for at least 30 days, which means that within 30 days, you are able to obtain POI details by Place ID from Place Details API. After 30 days, the interpolated POIs(both Place ID and details) may expire and inaccessible from the Places Details API.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn place_id(&self) -> ::std::option::Option<&str> {
        self.place_id.as_deref()
    }
    /// <p>The Amazon Location categories that describe the Place.</p>
    /// <p>For more information about using categories, including a list of Amazon Location categories, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/category-filtering.html">Categories and filtering</a>, in the <i>Amazon Location Service Developer Guide</i>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.categories.is_none()`.
    pub fn categories(&self) -> &[::std::string::String] {
        self.categories.as_deref().unwrap_or_default()
    }
    /// <p>Categories from the data provider that describe the Place that are not mapped to any Amazon Location categories.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.supplemental_categories.is_none()`.
    pub fn supplemental_categories(&self) -> &[::std::string::String] {
        self.supplemental_categories.as_deref().unwrap_or_default()
    }
}
impl SearchForSuggestionsResult {
    /// Creates a new builder-style object to manufacture [`SearchForSuggestionsResult`](crate::types::SearchForSuggestionsResult).
    pub fn builder() -> crate::types::builders::SearchForSuggestionsResultBuilder {
        crate::types::builders::SearchForSuggestionsResultBuilder::default()
    }
}

/// A builder for [`SearchForSuggestionsResult`](crate::types::SearchForSuggestionsResult).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SearchForSuggestionsResultBuilder {
    pub(crate) text: ::std::option::Option<::std::string::String>,
    pub(crate) place_id: ::std::option::Option<::std::string::String>,
    pub(crate) categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) supplemental_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl SearchForSuggestionsResultBuilder {
    /// <p>The text of the place suggestion, typically formatted as an address string.</p>
    /// This field is required.
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.text = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The text of the place suggestion, typically formatted as an address string.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.text = input;
        self
    }
    /// <p>The text of the place suggestion, typically formatted as an address string.</p>
    pub fn get_text(&self) -> &::std::option::Option<::std::string::String> {
        &self.text
    }
    /// <p>The unique identifier of the Place. You can use this with the <code>GetPlace</code> operation to find the place again later, or to get full information for the Place.</p>
    /// <p>The <code>GetPlace</code> request must use the same <code>PlaceIndex</code> resource as the <code>SearchPlaceIndexForSuggestions</code> that generated the Place ID.</p><note>
    /// <p>For <code>SearchPlaceIndexForSuggestions</code> operations, the <code>PlaceId</code> is returned by place indexes that use Esri, Grab, or HERE as data providers.</p>
    /// </note>
    /// <p>While you can use PlaceID in subsequent requests, PlaceID is not intended to be a permanent identifier and the ID can change between consecutive API calls. Please see the following PlaceID behaviour for each data provider:</p>
    /// <ul>
    /// <li>
    /// <p>Esri: Place IDs will change every quarter at a minimum. The typical time period for these changes would be March, June, September, and December. Place IDs might also change between the typical quarterly change but that will be much less frequent.</p></li>
    /// <li>
    /// <p>HERE: We recommend that you cache data for no longer than a week to keep your data data fresh. You can assume that less than 1% ID shifts will release over release which is approximately 1 - 2 times per week.</p></li>
    /// <li>
    /// <p>Grab: Place IDs can expire or become invalid in the following situations.</p>
    /// <ul>
    /// <li>
    /// <p>Data operations: The POI may be removed from Grab POI database by Grab Map Ops based on the ground-truth, such as being closed in the real world, being detected as a duplicate POI, or having incorrect information. Grab will synchronize data to the Waypoint environment on weekly basis.</p></li>
    /// <li>
    /// <p>Interpolated POI: Interpolated POI is a temporary POI generated in real time when serving a request, and it will be marked as derived in the <code>place.result_type</code> field in the response. The information of interpolated POIs will be retained for at least 30 days, which means that within 30 days, you are able to obtain POI details by Place ID from Place Details API. After 30 days, the interpolated POIs(both Place ID and details) may expire and inaccessible from the Places Details API.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn place_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.place_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the Place. You can use this with the <code>GetPlace</code> operation to find the place again later, or to get full information for the Place.</p>
    /// <p>The <code>GetPlace</code> request must use the same <code>PlaceIndex</code> resource as the <code>SearchPlaceIndexForSuggestions</code> that generated the Place ID.</p><note>
    /// <p>For <code>SearchPlaceIndexForSuggestions</code> operations, the <code>PlaceId</code> is returned by place indexes that use Esri, Grab, or HERE as data providers.</p>
    /// </note>
    /// <p>While you can use PlaceID in subsequent requests, PlaceID is not intended to be a permanent identifier and the ID can change between consecutive API calls. Please see the following PlaceID behaviour for each data provider:</p>
    /// <ul>
    /// <li>
    /// <p>Esri: Place IDs will change every quarter at a minimum. The typical time period for these changes would be March, June, September, and December. Place IDs might also change between the typical quarterly change but that will be much less frequent.</p></li>
    /// <li>
    /// <p>HERE: We recommend that you cache data for no longer than a week to keep your data data fresh. You can assume that less than 1% ID shifts will release over release which is approximately 1 - 2 times per week.</p></li>
    /// <li>
    /// <p>Grab: Place IDs can expire or become invalid in the following situations.</p>
    /// <ul>
    /// <li>
    /// <p>Data operations: The POI may be removed from Grab POI database by Grab Map Ops based on the ground-truth, such as being closed in the real world, being detected as a duplicate POI, or having incorrect information. Grab will synchronize data to the Waypoint environment on weekly basis.</p></li>
    /// <li>
    /// <p>Interpolated POI: Interpolated POI is a temporary POI generated in real time when serving a request, and it will be marked as derived in the <code>place.result_type</code> field in the response. The information of interpolated POIs will be retained for at least 30 days, which means that within 30 days, you are able to obtain POI details by Place ID from Place Details API. After 30 days, the interpolated POIs(both Place ID and details) may expire and inaccessible from the Places Details API.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn set_place_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.place_id = input;
        self
    }
    /// <p>The unique identifier of the Place. You can use this with the <code>GetPlace</code> operation to find the place again later, or to get full information for the Place.</p>
    /// <p>The <code>GetPlace</code> request must use the same <code>PlaceIndex</code> resource as the <code>SearchPlaceIndexForSuggestions</code> that generated the Place ID.</p><note>
    /// <p>For <code>SearchPlaceIndexForSuggestions</code> operations, the <code>PlaceId</code> is returned by place indexes that use Esri, Grab, or HERE as data providers.</p>
    /// </note>
    /// <p>While you can use PlaceID in subsequent requests, PlaceID is not intended to be a permanent identifier and the ID can change between consecutive API calls. Please see the following PlaceID behaviour for each data provider:</p>
    /// <ul>
    /// <li>
    /// <p>Esri: Place IDs will change every quarter at a minimum. The typical time period for these changes would be March, June, September, and December. Place IDs might also change between the typical quarterly change but that will be much less frequent.</p></li>
    /// <li>
    /// <p>HERE: We recommend that you cache data for no longer than a week to keep your data data fresh. You can assume that less than 1% ID shifts will release over release which is approximately 1 - 2 times per week.</p></li>
    /// <li>
    /// <p>Grab: Place IDs can expire or become invalid in the following situations.</p>
    /// <ul>
    /// <li>
    /// <p>Data operations: The POI may be removed from Grab POI database by Grab Map Ops based on the ground-truth, such as being closed in the real world, being detected as a duplicate POI, or having incorrect information. Grab will synchronize data to the Waypoint environment on weekly basis.</p></li>
    /// <li>
    /// <p>Interpolated POI: Interpolated POI is a temporary POI generated in real time when serving a request, and it will be marked as derived in the <code>place.result_type</code> field in the response. The information of interpolated POIs will be retained for at least 30 days, which means that within 30 days, you are able to obtain POI details by Place ID from Place Details API. After 30 days, the interpolated POIs(both Place ID and details) may expire and inaccessible from the Places Details API.</p></li>
    /// </ul></li>
    /// </ul>
    pub fn get_place_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.place_id
    }
    /// Appends an item to `categories`.
    ///
    /// To override the contents of this collection use [`set_categories`](Self::set_categories).
    ///
    /// <p>The Amazon Location categories that describe the Place.</p>
    /// <p>For more information about using categories, including a list of Amazon Location categories, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/category-filtering.html">Categories and filtering</a>, in the <i>Amazon Location Service Developer Guide</i>.</p>
    pub fn categories(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.categories.unwrap_or_default();
        v.push(input.into());
        self.categories = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Location categories that describe the Place.</p>
    /// <p>For more information about using categories, including a list of Amazon Location categories, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/category-filtering.html">Categories and filtering</a>, in the <i>Amazon Location Service Developer Guide</i>.</p>
    pub fn set_categories(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.categories = input;
        self
    }
    /// <p>The Amazon Location categories that describe the Place.</p>
    /// <p>For more information about using categories, including a list of Amazon Location categories, see <a href="https://docs.aws.amazon.com/location/latest/developerguide/category-filtering.html">Categories and filtering</a>, in the <i>Amazon Location Service Developer Guide</i>.</p>
    pub fn get_categories(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.categories
    }
    /// Appends an item to `supplemental_categories`.
    ///
    /// To override the contents of this collection use [`set_supplemental_categories`](Self::set_supplemental_categories).
    ///
    /// <p>Categories from the data provider that describe the Place that are not mapped to any Amazon Location categories.</p>
    pub fn supplemental_categories(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.supplemental_categories.unwrap_or_default();
        v.push(input.into());
        self.supplemental_categories = ::std::option::Option::Some(v);
        self
    }
    /// <p>Categories from the data provider that describe the Place that are not mapped to any Amazon Location categories.</p>
    pub fn set_supplemental_categories(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.supplemental_categories = input;
        self
    }
    /// <p>Categories from the data provider that describe the Place that are not mapped to any Amazon Location categories.</p>
    pub fn get_supplemental_categories(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.supplemental_categories
    }
    /// Consumes the builder and constructs a [`SearchForSuggestionsResult`](crate::types::SearchForSuggestionsResult).
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](crate::types::builders::SearchForSuggestionsResultBuilder::text)
    pub fn build(self) -> ::std::result::Result<crate::types::SearchForSuggestionsResult, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::SearchForSuggestionsResult {
            text: self.text.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "text",
                    "text was not specified but it is required when building SearchForSuggestionsResult",
                )
            })?,
            place_id: self.place_id,
            categories: self.categories,
            supplemental_categories: self.supplemental_categories,
        })
    }
}
