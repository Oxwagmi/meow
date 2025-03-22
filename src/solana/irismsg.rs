use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;
use solana_sdk::signature::Signature;
use std::time::Duration;
use tokio::time::sleep;

const IRIS_API_SANDOX_URL: &str = "https://iris-api-sandbox.circle.com";
const IRIS_API_URL: &str = "https://iris-api.circle.com/";
const SOLANA_SRC_DOMAIN_ID: u32 = 5;

#[derive(Debug)]
pub struct AttestationData {
    pub message: String,
    pub attestation: String,
}

pub async fn get_messages(tx_hash: &Signature, mainnet: bool) -> Result<AttestationData> {
    let client = Client::new();
    let url = if mainnet {
        format!(
            "{}/messages/{}/{}",
            IRIS_API_URL, SOLANA_SRC_DOMAIN_ID, tx_hash
        )
    } else {
        format!(
            "{}/messages/{}/{}",
            IRIS_API_SANDOX_URL, SOLANA_SRC_DOMAIN_ID, tx_hash
        )
    };

    println!("🔍 Fetching messages for tx: {}", tx_hash);

    for attempt in 1..=5 {
        let response = client
            .get(&url)
            .send()
            .await
            .context("Failed to send HTTP request")?;

        let attestation_response: Value = response
            .json()
            .await
            .context("Failed to parse JSON response")?;

        if let Some(error) = attestation_response.get("error") {
            let error_msg = error.as_str().unwrap_or_default();
            if error_msg.contains("Transaction hash not found") {
                println!(
                    "⚠️ Transaction not indexed yet. Retrying {}/5 in 15s...",
                    attempt
                );
                sleep(Duration::from_secs(15)).await;
                continue;
            } else {
                return Err(anyhow::anyhow!("❌ Received error response: {:?}", error));
            }
        }

        if let Some(messages) = attestation_response.get("messages") {
            if let Some(msg) = messages.get(0) {
                if let Some(attestation) = msg.get("attestation") {
                    if attestation != "PENDING" {
                        let message = msg
                            .get("message")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_string();

                        let attestation_str = attestation.as_str().unwrap_or("").to_string();

                        println!("✅ Attestation received!");
                        return Ok(AttestationData {
                            message,
                            attestation: attestation_str,
                        });
                    }
                }
            }
        }

        println!(
            "⌛ Attestation is still pending... retrying {}/5 in 10s",
            attempt
        );
        sleep(Duration::from_secs(10)).await;
    }

    Err(anyhow::anyhow!(
        "❌ Attestation not received after multiple retries"
    ))
}
