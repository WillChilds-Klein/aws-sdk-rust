// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_cis_scan_report_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_cis_scan_report::GetCisScanReportInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.scan_arn {
        object.key("scanArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_accounts {
        let mut array_3 = object.key("targetAccounts").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}
