// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
Purpose

Shows how to use rustls, hyper-rustls, and webpki-roots to set the minimum TLS version to 1.3 for
outgoing connections.
Then, uses TLS 1.3 to make a sample call to AWS SecretsManager as proof of concept.

This example assumes you have set up environment variables for authentication.

*/
use aws_config::BehaviorVersion;

use aws_smithy_experimental::hyper_1_0::{CryptoMode, HyperClientBuilder};

pub async fn connect_via_tls_13() -> Result<(), aws_sdk_secretsmanager::Error> {
    println!("Attempting to connect to SecretsManager using TLS 1.3: ");

    rustls_post_quantum::provider().install_default().unwrap();

    let http_client = HyperClientBuilder::new()
        .crypto_mode(CryptoMode::PQ)
        .build_https();

    let shared_conf = aws_config::defaults(BehaviorVersion::latest())
        .http_client(http_client)
        .load()
        .await;

    let client = aws_sdk_secretsmanager::Client::new(&shared_conf);
    let response = client.list_secrets().send().await?;

    println!("{:?}", response);

    Ok(())
}
