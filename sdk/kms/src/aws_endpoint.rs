// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
            .id("aws")
            .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
            .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                uri_template: "kms.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope: aws_endpoint::CredentialScope::builder().build(),
            })
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .endpoint(
                "ProdFips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-central-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-central-2")
                        .build(),
                },
            )
            .endpoint(
                "af-south-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "af-south-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.af-south-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("af-south-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-east-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-east-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-northeast-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-northeast-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-northeast-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-northeast-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-northeast-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-northeast-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-northeast-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-northeast-2")
                        .build(),
                },
            )
            .endpoint(
                "ap-northeast-3",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-northeast-3-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-northeast-3.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-northeast-3")
                        .build(),
                },
            )
            .endpoint(
                "ap-south-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-south-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-south-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-south-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-south-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-south-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-south-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-south-2")
                        .build(),
                },
            )
            .endpoint(
                "ap-southeast-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-southeast-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-southeast-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-southeast-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-southeast-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-southeast-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-southeast-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-southeast-2")
                        .build(),
                },
            )
            .endpoint(
                "ap-southeast-3",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ap-southeast-3-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-southeast-3.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-southeast-3")
                        .build(),
                },
            )
            .endpoint(
                "ap-southeast-4-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ap-southeast-4.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-southeast-4")
                        .build(),
                },
            )
            .endpoint(
                "ca-central-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "ca-central-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.ca-central-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ca-central-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-central-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-central-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-central-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-central-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-central-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-central-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-central-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-central-2")
                        .build(),
                },
            )
            .endpoint(
                "eu-north-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-north-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-north-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-north-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-south-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-south-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-south-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-south-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-south-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-south-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-south-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-south-2")
                        .build(),
                },
            )
            .endpoint(
                "eu-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-west-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-west-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-west-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-west-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-west-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-west-2")
                        .build(),
                },
            )
            .endpoint(
                "eu-west-3",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "eu-west-3-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.eu-west-3.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-west-3")
                        .build(),
                },
            )
            .endpoint(
                "me-central-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "me-central-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.me-central-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("me-central-1")
                        .build(),
                },
            )
            .endpoint(
                "me-south-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "me-south-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.me-south-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("me-south-1")
                        .build(),
                },
            )
            .endpoint(
                "sa-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "sa-east-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.sa-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("sa-east-1")
                        .build(),
                },
            )
            .endpoint(
                "us-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "us-east-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.us-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .endpoint(
                "us-east-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "us-east-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.us-east-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-2")
                        .build(),
                },
            )
            .endpoint(
                "us-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "us-west-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.us-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-west-1")
                        .build(),
                },
            )
            .endpoint(
                "us-west-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                },
            )
            .endpoint(
                "us-west-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms-fips.us-west-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-west-2")
                        .build(),
                },
            )
            .build()
            .expect("invalid partition"),
        vec![
            aws_endpoint::Partition::builder()
                .id("aws-cn")
                .region_regex(r#"^cn\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso")
                .region_regex(r#"^us\-iso\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.c2s.ic.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .endpoint(
                    "ProdFips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-iso-east-1.c2s.ic.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-iso-east-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-iso-east-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms.{region}.c2s.ic.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder().build(),
                    },
                )
                .endpoint(
                    "us-iso-east-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-iso-east-1.c2s.ic.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-iso-east-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-iso-west-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms.{region}.c2s.ic.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder().build(),
                    },
                )
                .endpoint(
                    "us-iso-west-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-iso-west-1.c2s.ic.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-iso-west-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso-b")
                .region_regex(r#"^us\-isob\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.sc2s.sgov.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .endpoint(
                    "ProdFips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-isob-east-1.sc2s.sgov.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-isob-east-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-isob-east-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms.{region}.sc2s.sgov.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder().build(),
                    },
                )
                .endpoint(
                    "us-isob-east-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-isob-east-1.sc2s.sgov.gov",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-isob-east-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-us-gov")
                .region_regex(r#"^us\-gov\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "kms.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .endpoint(
                    "ProdFips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-gov-west-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-gov-east-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms.{region}.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder().build(),
                    },
                )
                .endpoint(
                    "us-gov-east-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-gov-east-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-east-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-gov-west-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms.{region}.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder().build(),
                    },
                )
                .endpoint(
                    "us-gov-west-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "kms-fips.us-gov-west-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
        ],
    )
}
