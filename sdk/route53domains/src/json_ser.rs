// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_accept_domain_transfer_from_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AcceptDomainTransferFromAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.password {
        object.key("Password").string(var_2.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_delegation_signer_to_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateDelegationSignerToDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_3) = &input.domain_name {
        object.key("DomainName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.signing_attributes {
        #[allow(unused_mut)]
        let mut object_5 = object.key("SigningAttributes").start_object();
        crate::json_ser::serialize_structure_crate_model_dnssec_signing_attributes(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_cancel_domain_transfer_to_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelDomainTransferToAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.domain_name {
        object.key("DomainName").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_check_domain_availability_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CheckDomainAvailabilityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.domain_name {
        object.key("DomainName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.idn_lang_code {
        object.key("IdnLangCode").string(var_8.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_check_domain_transferability_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CheckDomainTransferabilityInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.domain_name {
        object.key("DomainName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.auth_code {
        object.key("AuthCode").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.domain_name {
        object.key("DomainName").string(var_11.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_tags_for_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTagsForDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.domain_name {
        object.key("DomainName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.tags_to_delete {
        let mut array_14 = object.key("TagsToDelete").start_array();
        for item_15 in var_13 {
            {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_domain_auto_renew_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableDomainAutoRenewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.domain_name {
        object.key("DomainName").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disable_domain_transfer_lock_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisableDomainTransferLockInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.domain_name {
        object.key("DomainName").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_delegation_signer_from_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateDelegationSignerFromDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.domain_name {
        object.key("DomainName").string(var_18.as_str());
    }
    if let Some(var_19) = &input.id {
        object.key("Id").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_domain_auto_renew_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableDomainAutoRenewInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.domain_name {
        object.key("DomainName").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_enable_domain_transfer_lock_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::EnableDomainTransferLockInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.domain_name {
        object.key("DomainName").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_contact_reachability_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetContactReachabilityStatusInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.domain_name {
        object.key("domainName").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_domain_detail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDomainDetailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.domain_name {
        object.key("DomainName").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_domain_suggestions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDomainSuggestionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.domain_name {
        object.key("DomainName").string(var_24.as_str());
    }
    {
        object.key("SuggestionCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.suggestion_count).into()),
        );
    }
    if let Some(var_25) = &input.only_available {
        object.key("OnlyAvailable").boolean(*var_25);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_operation_detail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetOperationDetailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.operation_id {
        object.key("OperationId").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_domains_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListDomainsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.filter_conditions {
        let mut array_28 = object.key("FilterConditions").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter_condition(
                    &mut object_30,
                    item_29,
                )?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    if let Some(var_31) = &input.sort_condition {
        #[allow(unused_mut)]
        let mut object_32 = object.key("SortCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_sort_condition(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.marker {
        object.key("Marker").string(var_33.as_str());
    }
    if let Some(var_34) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_operations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListOperationsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.submitted_since {
        object
            .key("SubmittedSince")
            .date_time(var_35, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_36) = &input.marker {
        object.key("Marker").string(var_36.as_str());
    }
    if let Some(var_37) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.status {
        let mut array_39 = object.key("Status").start_array();
        for item_40 in var_38 {
            {
                array_39.value().string(item_40.as_str());
            }
        }
        array_39.finish();
    }
    if let Some(var_41) = &input.r#type {
        let mut array_42 = object.key("Type").start_array();
        for item_43 in var_41 {
            {
                array_42.value().string(item_43.as_str());
            }
        }
        array_42.finish();
    }
    if let Some(var_44) = &input.sort_by {
        object.key("SortBy").string(var_44.as_str());
    }
    if let Some(var_45) = &input.sort_order {
        object.key("SortOrder").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_prices_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPricesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.tld {
        object.key("Tld").string(var_46.as_str());
    }
    if let Some(var_47) = &input.marker {
        object.key("Marker").string(var_47.as_str());
    }
    if let Some(var_48) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_48).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsForDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_49) = &input.domain_name {
        object.key("DomainName").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_push_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PushDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.domain_name {
        object.key("DomainName").string(var_50.as_str());
    }
    if let Some(var_51) = &input.target {
        object.key("Target").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.domain_name {
        object.key("DomainName").string(var_52.as_str());
    }
    if let Some(var_53) = &input.idn_lang_code {
        object.key("IdnLangCode").string(var_53.as_str());
    }
    if let Some(var_54) = &input.duration_in_years {
        object.key("DurationInYears").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_54).into()),
        );
    }
    if let Some(var_55) = &input.auto_renew {
        object.key("AutoRenew").boolean(*var_55);
    }
    if let Some(var_56) = &input.admin_contact {
        #[allow(unused_mut)]
        let mut object_57 = object.key("AdminContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.registrant_contact {
        #[allow(unused_mut)]
        let mut object_59 = object.key("RegistrantContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_59, var_58)?;
        object_59.finish();
    }
    if let Some(var_60) = &input.tech_contact {
        #[allow(unused_mut)]
        let mut object_61 = object.key("TechContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_61, var_60)?;
        object_61.finish();
    }
    if let Some(var_62) = &input.privacy_protect_admin_contact {
        object.key("PrivacyProtectAdminContact").boolean(*var_62);
    }
    if let Some(var_63) = &input.privacy_protect_registrant_contact {
        object
            .key("PrivacyProtectRegistrantContact")
            .boolean(*var_63);
    }
    if let Some(var_64) = &input.privacy_protect_tech_contact {
        object.key("PrivacyProtectTechContact").boolean(*var_64);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reject_domain_transfer_from_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RejectDomainTransferFromAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.domain_name {
        object.key("DomainName").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_renew_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RenewDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.domain_name {
        object.key("DomainName").string(var_66.as_str());
    }
    if let Some(var_67) = &input.duration_in_years {
        object.key("DurationInYears").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_67).into()),
        );
    }
    {
        object.key("CurrentExpiryYear").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.current_expiry_year).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resend_contact_reachability_email_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResendContactReachabilityEmailInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_68) = &input.domain_name {
        object.key("domainName").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resend_operation_authorization_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResendOperationAuthorizationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_69) = &input.operation_id {
        object.key("OperationId").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_retrieve_domain_auth_code_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RetrieveDomainAuthCodeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.domain_name {
        object.key("DomainName").string(var_70.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_transfer_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TransferDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.domain_name {
        object.key("DomainName").string(var_71.as_str());
    }
    if let Some(var_72) = &input.idn_lang_code {
        object.key("IdnLangCode").string(var_72.as_str());
    }
    if let Some(var_73) = &input.duration_in_years {
        object.key("DurationInYears").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_73).into()),
        );
    }
    if let Some(var_74) = &input.nameservers {
        let mut array_75 = object.key("Nameservers").start_array();
        for item_76 in var_74 {
            {
                #[allow(unused_mut)]
                let mut object_77 = array_75.value().start_object();
                crate::json_ser::serialize_structure_crate_model_nameserver(
                    &mut object_77,
                    item_76,
                )?;
                object_77.finish();
            }
        }
        array_75.finish();
    }
    if let Some(var_78) = &input.auth_code {
        object.key("AuthCode").string(var_78.as_str());
    }
    if let Some(var_79) = &input.auto_renew {
        object.key("AutoRenew").boolean(*var_79);
    }
    if let Some(var_80) = &input.admin_contact {
        #[allow(unused_mut)]
        let mut object_81 = object.key("AdminContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_81, var_80)?;
        object_81.finish();
    }
    if let Some(var_82) = &input.registrant_contact {
        #[allow(unused_mut)]
        let mut object_83 = object.key("RegistrantContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_83, var_82)?;
        object_83.finish();
    }
    if let Some(var_84) = &input.tech_contact {
        #[allow(unused_mut)]
        let mut object_85 = object.key("TechContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_85, var_84)?;
        object_85.finish();
    }
    if let Some(var_86) = &input.privacy_protect_admin_contact {
        object.key("PrivacyProtectAdminContact").boolean(*var_86);
    }
    if let Some(var_87) = &input.privacy_protect_registrant_contact {
        object
            .key("PrivacyProtectRegistrantContact")
            .boolean(*var_87);
    }
    if let Some(var_88) = &input.privacy_protect_tech_contact {
        object.key("PrivacyProtectTechContact").boolean(*var_88);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_transfer_domain_to_another_aws_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TransferDomainToAnotherAwsAccountInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.domain_name {
        object.key("DomainName").string(var_89.as_str());
    }
    if let Some(var_90) = &input.account_id {
        object.key("AccountId").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_contact_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainContactInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.domain_name {
        object.key("DomainName").string(var_91.as_str());
    }
    if let Some(var_92) = &input.admin_contact {
        #[allow(unused_mut)]
        let mut object_93 = object.key("AdminContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_93, var_92)?;
        object_93.finish();
    }
    if let Some(var_94) = &input.registrant_contact {
        #[allow(unused_mut)]
        let mut object_95 = object.key("RegistrantContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_95, var_94)?;
        object_95.finish();
    }
    if let Some(var_96) = &input.tech_contact {
        #[allow(unused_mut)]
        let mut object_97 = object.key("TechContact").start_object();
        crate::json_ser::serialize_structure_crate_model_contact_detail(&mut object_97, var_96)?;
        object_97.finish();
    }
    if let Some(var_98) = &input.consent {
        #[allow(unused_mut)]
        let mut object_99 = object.key("Consent").start_object();
        crate::json_ser::serialize_structure_crate_model_consent(&mut object_99, var_98)?;
        object_99.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_contact_privacy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainContactPrivacyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.domain_name {
        object.key("DomainName").string(var_100.as_str());
    }
    if let Some(var_101) = &input.admin_privacy {
        object.key("AdminPrivacy").boolean(*var_101);
    }
    if let Some(var_102) = &input.registrant_privacy {
        object.key("RegistrantPrivacy").boolean(*var_102);
    }
    if let Some(var_103) = &input.tech_privacy {
        object.key("TechPrivacy").boolean(*var_103);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_nameservers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDomainNameserversInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_104) = &input.domain_name {
        object.key("DomainName").string(var_104.as_str());
    }
    if let Some(var_105) = &input.fi_auth_key {
        object.key("FIAuthKey").string(var_105.as_str());
    }
    if let Some(var_106) = &input.nameservers {
        let mut array_107 = object.key("Nameservers").start_array();
        for item_108 in var_106 {
            {
                #[allow(unused_mut)]
                let mut object_109 = array_107.value().start_object();
                crate::json_ser::serialize_structure_crate_model_nameserver(
                    &mut object_109,
                    item_108,
                )?;
                object_109.finish();
            }
        }
        array_107.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_tags_for_domain_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTagsForDomainInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_110) = &input.domain_name {
        object.key("DomainName").string(var_110.as_str());
    }
    if let Some(var_111) = &input.tags_to_update {
        let mut array_112 = object.key("TagsToUpdate").start_array();
        for item_113 in var_111 {
            {
                #[allow(unused_mut)]
                let mut object_114 = array_112.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_114, item_113)?;
                object_114.finish();
            }
        }
        array_112.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_view_billing_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ViewBillingInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_115) = &input.start {
        object
            .key("Start")
            .date_time(var_115, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_116) = &input.end {
        object
            .key("End")
            .date_time(var_116, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_117) = &input.marker {
        object.key("Marker").string(var_117.as_str());
    }
    if let Some(var_118) = &input.max_items {
        object.key("MaxItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_118).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dnssec_signing_attributes(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DnssecSigningAttributes,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_119) = &input.algorithm {
        object.key("Algorithm").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_119).into()),
        );
    }
    if let Some(var_120) = &input.flags {
        object.key("Flags").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_120).into()),
        );
    }
    if let Some(var_121) = &input.public_key {
        object.key("PublicKey").string(var_121.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FilterCondition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_122) = &input.name {
        object.key("Name").string(var_122.as_str());
    }
    if let Some(var_123) = &input.operator {
        object.key("Operator").string(var_123.as_str());
    }
    if let Some(var_124) = &input.values {
        let mut array_125 = object.key("Values").start_array();
        for item_126 in var_124 {
            {
                array_125.value().string(item_126.as_str());
            }
        }
        array_125.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_sort_condition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SortCondition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.name {
        object.key("Name").string(var_127.as_str());
    }
    if let Some(var_128) = &input.sort_order {
        object.key("SortOrder").string(var_128.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_contact_detail(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ContactDetail,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.first_name {
        object.key("FirstName").string(var_129.as_str());
    }
    if let Some(var_130) = &input.last_name {
        object.key("LastName").string(var_130.as_str());
    }
    if let Some(var_131) = &input.contact_type {
        object.key("ContactType").string(var_131.as_str());
    }
    if let Some(var_132) = &input.organization_name {
        object.key("OrganizationName").string(var_132.as_str());
    }
    if let Some(var_133) = &input.address_line1 {
        object.key("AddressLine1").string(var_133.as_str());
    }
    if let Some(var_134) = &input.address_line2 {
        object.key("AddressLine2").string(var_134.as_str());
    }
    if let Some(var_135) = &input.city {
        object.key("City").string(var_135.as_str());
    }
    if let Some(var_136) = &input.state {
        object.key("State").string(var_136.as_str());
    }
    if let Some(var_137) = &input.country_code {
        object.key("CountryCode").string(var_137.as_str());
    }
    if let Some(var_138) = &input.zip_code {
        object.key("ZipCode").string(var_138.as_str());
    }
    if let Some(var_139) = &input.phone_number {
        object.key("PhoneNumber").string(var_139.as_str());
    }
    if let Some(var_140) = &input.email {
        object.key("Email").string(var_140.as_str());
    }
    if let Some(var_141) = &input.fax {
        object.key("Fax").string(var_141.as_str());
    }
    if let Some(var_142) = &input.extra_params {
        let mut array_143 = object.key("ExtraParams").start_array();
        for item_144 in var_142 {
            {
                #[allow(unused_mut)]
                let mut object_145 = array_143.value().start_object();
                crate::json_ser::serialize_structure_crate_model_extra_param(
                    &mut object_145,
                    item_144,
                )?;
                object_145.finish();
            }
        }
        array_143.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_nameserver(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Nameserver,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_146) = &input.name {
        object.key("Name").string(var_146.as_str());
    }
    if let Some(var_147) = &input.glue_ips {
        let mut array_148 = object.key("GlueIps").start_array();
        for item_149 in var_147 {
            {
                array_148.value().string(item_149.as_str());
            }
        }
        array_148.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_consent(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Consent,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("MaxPrice").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.max_price).into()),
        );
    }
    if let Some(var_150) = &input.currency {
        object.key("Currency").string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_151) = &input.key {
        object.key("Key").string(var_151.as_str());
    }
    if let Some(var_152) = &input.value {
        object.key("Value").string(var_152.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_extra_param(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ExtraParam,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_153) = &input.name {
        object.key("Name").string(var_153.as_str());
    }
    if let Some(var_154) = &input.value {
        object.key("Value").string(var_154.as_str());
    }
    Ok(())
}
