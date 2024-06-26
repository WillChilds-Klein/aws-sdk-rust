// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_service_managed_ec2_instance_capabilities(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ServiceManagedEc2InstanceCapabilities,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.v_cpu_count {
        #[allow(unused_mut)]
        let mut object_2 = object.key("vCpuCount").start_object();
        crate::protocol_serde::shape_v_cpu_count_range::ser_v_cpu_count_range(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.memory_mib {
        #[allow(unused_mut)]
        let mut object_4 = object.key("memoryMiB").start_object();
        crate::protocol_serde::shape_memory_mib_range::ser_memory_mib_range(&mut object_4, var_3)?;
        object_4.finish();
    }
    {
        object.key("osFamily").string(input.os_family.as_str());
    }
    {
        object.key("cpuArchitectureType").string(input.cpu_architecture_type.as_str());
    }
    if let Some(var_5) = &input.root_ebs_volume {
        #[allow(unused_mut)]
        let mut object_6 = object.key("rootEbsVolume").start_object();
        crate::protocol_serde::shape_ec2_ebs_volume::ser_ec2_ebs_volume(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.allowed_instance_types {
        let mut array_8 = object.key("allowedInstanceTypes").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.excluded_instance_types {
        let mut array_11 = object.key("excludedInstanceTypes").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.custom_amounts {
        let mut array_14 = object.key("customAmounts").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_fleet_amount_capability::ser_fleet_amount_capability(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.custom_attributes {
        let mut array_18 = object.key("customAttributes").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_fleet_attribute_capability::ser_fleet_attribute_capability(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub(crate) fn de_service_managed_ec2_instance_capabilities<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ServiceManagedEc2InstanceCapabilities>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ServiceManagedEc2InstanceCapabilitiesBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "vCpuCount" => {
                            builder = builder.set_v_cpu_count(crate::protocol_serde::shape_v_cpu_count_range::de_v_cpu_count_range(tokens)?);
                        }
                        "memoryMiB" => {
                            builder = builder.set_memory_mib(crate::protocol_serde::shape_memory_mib_range::de_memory_mib_range(tokens)?);
                        }
                        "osFamily" => {
                            builder = builder.set_os_family(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::ServiceManagedFleetOperatingSystemFamily::from(u.as_ref()))
                                    })
                                    .transpose()?,
                            );
                        }
                        "cpuArchitectureType" => {
                            builder = builder.set_cpu_architecture_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::CpuArchitectureType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "rootEbsVolume" => {
                            builder = builder.set_root_ebs_volume(crate::protocol_serde::shape_ec2_ebs_volume::de_ec2_ebs_volume(tokens)?);
                        }
                        "allowedInstanceTypes" => {
                            builder = builder.set_allowed_instance_types(crate::protocol_serde::shape_instance_types::de_instance_types(tokens)?);
                        }
                        "excludedInstanceTypes" => {
                            builder = builder.set_excluded_instance_types(crate::protocol_serde::shape_instance_types::de_instance_types(tokens)?);
                        }
                        "customAmounts" => {
                            builder = builder.set_custom_amounts(
                                crate::protocol_serde::shape_custom_fleet_amount_capabilities::de_custom_fleet_amount_capabilities(tokens)?,
                            );
                        }
                        "customAttributes" => {
                            builder = builder.set_custom_attributes(
                                crate::protocol_serde::shape_custom_fleet_attribute_capabilities::de_custom_fleet_attribute_capabilities(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(
                crate::serde_util::service_managed_ec2_instance_capabilities_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
