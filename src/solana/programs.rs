use anchor_client::solana_client::client_error::reqwest::Error;
use anchor_client::solana_sdk::pubkey::Pubkey;
use hex;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_program;
// use solana_sdk::system_instruction;
use super::svm_manager::{SOLANA_MANAGER, SolanaManager};
use anyhow::{Result, anyhow};
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use std::str::FromStr;
use token_messenger_minter::accounts::DepositForBurnContext;
use token_messenger_minter::instruction::DepositForBurn;
use token_messenger_minter::token_messenger::DepositForBurnParams;
// use message_transmitter::Counter;
// use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ProgramAddress {
    pub public_key: Pubkey,
    pub bump: u8,
}

#[derive(Debug)]
pub struct DepositForBurnPDAs {
    pub message_transmitter_account: Pubkey,
    pub token_messenger_account: Pubkey,
    pub token_minter_account: Pubkey,
    pub local_token: Pubkey,
    pub remote_token_messenger_key: Pubkey,
    pub authority_pda: Pubkey,
    pub event_authority: Pubkey,
}

pub fn find_program_address(
    label: &str,
    program_id: &Pubkey,
    extra_seeds: Option<Vec<&[u8]>>,
) -> Result<ProgramAddress, Error> {
    let mut seeds = vec![label.as_bytes()];

    if let Some(extra) = extra_seeds {
        seeds.extend(extra);
    }

    let (public_key, bump) = Pubkey::find_program_address(&seeds, program_id);

    Ok(ProgramAddress { public_key, bump })
}

pub fn get_deposit_for_burn_pdas(
    message_transmitter_program_id: &Pubkey,
    token_messenger_minter_program_id: &Pubkey,
    usdc_address: &Pubkey,
    destination_domain: u32,
) -> Result<DepositForBurnPDAs> {
    Ok(DepositForBurnPDAs {
        message_transmitter_account: find_program_address(
            "message_transmitter",
            message_transmitter_program_id,
            None,
        )?
        .public_key,
        token_messenger_account: find_program_address(
            "token_messenger",
            token_messenger_minter_program_id,
            None,
        )?
        .public_key,
        token_minter_account: find_program_address(
            "token_minter",
            token_messenger_minter_program_id,
            None,
        )?
        .public_key,
        local_token: find_program_address(
            "local_token",
            token_messenger_minter_program_id,
            Some(vec![usdc_address.as_ref()]),
        )?
        .public_key,
        remote_token_messenger_key: find_program_address(
            "remote_token_messenger",
            token_messenger_minter_program_id,
            Some(vec![destination_domain.to_string().as_bytes()]),
        )?
        .public_key,
        authority_pda: find_program_address(
            "sender_authority",
            token_messenger_minter_program_id,
            None,
        )?
        .public_key,
        event_authority: find_program_address(
            "event_authority",
            token_messenger_minter_program_id,
            None,
        )?
        .public_key,
    })
}

pub async fn call_deposit_for_burn(
    destination_domain: u32,
    mint_recipient_address: &str,
    amount_in_usdc: u64,
    mainnet: bool,
    safe_format: bool,
) -> Result<Signature> {
    let manager: &SolanaManager = SOLANA_MANAGER.get().unwrap();
    let spl_token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")?;
    let payer = manager.payer();
    let mint_recipient = evm_address_to_pubkey(mint_recipient_address);
    let client = manager.client();
    let usdc_address = if mainnet {
        Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?
    } else {
        Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU")?
    };
    let user_token_account =
        ensure_token_account_exists(manager, &usdc_address, &spl_token_program).await?;
    let ev_auth = Pubkey::from_str("CNfZLeeL4RUxwfPnjA3tLiQt4y43jp4V7bMpga673jf9")?;

    let message_transmitter = client.program(Pubkey::from_str(
        "CCTPmbSD7gX1bxKPAmg77w8oFzNFpaQiQUWD43TKaecd",
    )?)?;
    let token_messenger_minter = client.program(Pubkey::from_str(
        "CCTPiPYPc6AsJuwueEnWgSgucamXDZwBd53dQ11YiKX3",
    )?)?;

    let pdas = get_deposit_for_burn_pdas(
        &message_transmitter.id(),
        &token_messenger_minter.id(),
        &usdc_address,
        destination_domain,
    )?;

    let message_sent_event_account_keypair = Keypair::new();

    const MAX_USDC_AMOUNT: u64 = 100;
    const USDC_DECIMALS: u64 = 1_000_000;

    if amount_in_usdc == 0 {
        return Err(anyhow::anyhow!("Transfer amount must be greater than zero"));
    }
    if safe_format && amount_in_usdc > MAX_USDC_AMOUNT {
        return Err(anyhow::anyhow!(
            "Transfer amount exceeds the 100 USDC limit for safety"
        ));
    }
    let amount_formated = amount_in_usdc
        .checked_mul(USDC_DECIMALS)
        .ok_or_else(|| anyhow!("Amount overflow during conversion"))?;

    println!(
        "  message_transmitter_account: {}",
        pdas.message_transmitter_account
    );
    println!(
        "  token_messenger_account: {}",
        pdas.token_messenger_account
    );
    println!("  token_minter_account: {}", pdas.token_minter_account);
    println!("  local_token: {}", pdas.local_token);
    println!(
        "  remote_token_messenger_key: {}",
        pdas.remote_token_messenger_key
    );
    println!("  authority_pda: {}", pdas.authority_pda);
    println!("  event_authority_pda: {}", pdas.event_authority);
    println!("Accounts:");
    println!("  payer: {}", payer.pubkey());
    println!("  burn_token_account: {}", user_token_account);
    println!("  burn_token_mint: {}", usdc_address);
    println!(
        "  message_sent_event_data: {}",
        message_sent_event_account_keypair.pubkey()
    );
    println!("  mint_recipient: {}", mint_recipient);

    let tx = token_messenger_minter
        .request()
        .signer(&message_sent_event_account_keypair)
        .accounts(DepositForBurnContext {
            owner: payer.pubkey(),
            event_rent_payer: payer.pubkey(),
            sender_authority_pda: pdas.authority_pda,
            burn_token_account: user_token_account,
            message_transmitter: pdas.message_transmitter_account,
            token_messenger: pdas.token_messenger_account,
            remote_token_messenger: pdas.remote_token_messenger_key,
            token_minter: pdas.token_minter_account,
            local_token: pdas.local_token,
            burn_token_mint: usdc_address,
            message_sent_event_data: message_sent_event_account_keypair.pubkey(),
            message_transmitter_program: message_transmitter.id(),
            token_messenger_minter_program: token_messenger_minter.id(),
            token_program: spl_token_program,
            event_authority: ev_auth,
            program: token_messenger_minter.id(),
            system_program: system_program::ID,
        })
        .args(DepositForBurn {
            params: DepositForBurnParams {
                amount: if safe_format {
                    amount_formated
                } else {
                    amount_in_usdc
                }, // 1 = 0.000001 USDC
                destination_domain: destination_domain,
                mint_recipient,
            },
        })
        .send()
        .await
        .unwrap();

    println!("DepositForBurn transaction sent: {:?}", tx);

    Ok(tx)
}

pub fn evm_address_to_pubkey(address: &str) -> Pubkey {
    let address_trimmed = address.trim_start_matches("0x");
    let evm_bytes = hex::decode(address_trimmed).expect("Invalid hex input");
    assert_eq!(evm_bytes.len(), 20, "EVM address must be 20 bytes");
    let mut padded_bytes = vec![0u8; 12]; // 12 bytes of `0x00`
    padded_bytes.extend(evm_bytes);
    Pubkey::new_from_array(padded_bytes.try_into().expect("Failed to create Pubkey"))
}
pub async fn ensure_token_account_exists(
    manager: &SolanaManager,
    mint: &Pubkey,
    spl_token_program: &Pubkey,
) -> Result<Pubkey> {
    let associated_token_account = get_associated_token_address(&manager.payer.pubkey(), mint);
    let account_exists = manager
        .rpc_client
        .get_account(&associated_token_account)
        .is_ok();

    if !account_exists {
        println!(
            "⚠️ Token account does not exist, creating: {}",
            associated_token_account
        );

        let create_account_ix = create_associated_token_account(
            &manager.payer.pubkey(), // Funding payer
            &manager.payer.pubkey(), // Token account owner
            mint,
            spl_token_program,
        );

        let transaction = Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&manager.payer.pubkey()),
            &[manager.payer.as_ref()],
            manager
                .rpc_client
                .get_latest_blockhash()
                .expect("Failed to get recent blockhash"),
        );

        manager
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| anyhow!("Failed to create associated token account: {}", e))?;

        println!(
            "✅ Token account created successfully: {}",
            associated_token_account
        );
    } else {
        println!(
            "✅ Token account already exists: {}",
            associated_token_account
        );
    }

    Ok(associated_token_account)
}
