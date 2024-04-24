// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Purpose

Shows how to use rustls, hyper-rustls, and webpki-roots to set the minimum TLS version to 1.3 for
outgoing connections.
Then, uses TLS 1.3 to make a sample call to AWS Key Management Service (AWS KMS) as proof of concept.

This example assumes you have set up environment variables for authentication.

*/
//use aws_config::BehaviorVersion;
use aws_smithy_runtime_api::client::behavior_version::BehaviorVersion;
//use aws_sdk_kms::Error;

use aws_smithy_experimental::hyper_1_0::{CryptoMode, HyperClientBuilder};

pub async fn connect_via_tls_13() -> Result<(), aws_sdk_kms::Error> {
    println!("Attempting to connect to KMS using TLS 1.3: ");

    rustls_post_quantum::provider().install_default().unwrap();

    // feature = crypto-aws-lc
    let http_client = HyperClientBuilder::new()
        .crypto_mode(CryptoMode::AwsLc)
        .build_https();

    let shared_conf = aws_config::defaults(BehaviorVersion::latest())
        .http_client(http_client)
        .load()
        .await;

    let kms_client = aws_sdk_kms::Client::new(&shared_conf);
    let response = kms_client
        .list_keys()
        .send()
        .await?;

    println!("{:?}", response);

    Ok(())
}
