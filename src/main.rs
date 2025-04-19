use anyhow::anyhow;
use clap::Parser;
use meow::app::{App, Command};
use meow::evm::claim::destination_balance_check;
use meow::evm::claim::evm_claim;
use meow::solana::constants::DestinationDomain;
use meow::solana::irismsg::get_messages;
use meow::solana::svm_manager::{SOLANA_MANAGER, SolanaManager, init_solana_manager};

// #[tokio::main(flavor = "multi_thread")]
#[tokio::main]
async fn main() {
    match dotenv::from_filename(".env") {
        Ok(_) => println!(".env file loaded successfully"),
        Err(err) => eprintln!("Failed to load .env file: {}", err),
    }

    let app = App::parse();

    match app.command {
        Command::BridgeSolanaUSDC {
            mainnet,
            safe_format_usdc,
            amount,
            to_chain,
            to,
        } => {
            let domain = DestinationDomain::from_str(&to_chain)
                .ok_or_else(|| anyhow!("Invalid chain: {}", to_chain))
                .unwrap();

            let fixed_domain = domain.as_u32();

            init_solana_manager(mainnet).unwrap();
            println!("CHECKING BALANCES OF BOTH SIDES BEFORE SENDING....");
            destination_balance_check(fixed_domain, mainnet).await;
            let manager: &SolanaManager = SOLANA_MANAGER.get().unwrap();
            let _ = manager.check_balance(0).await.unwrap();

            let deposit_for_burn_sig = meow::solana::programs::call_deposit_for_burn(
                fixed_domain,
                &to,
                amount,
                mainnet,
                safe_format_usdc,
            )
            .await
            .unwrap();
            let attestation_data = get_messages(&deposit_for_burn_sig, mainnet).await.unwrap();
            evm_claim(
                &attestation_data.message,
                &attestation_data.attestation,
                fixed_domain,
                mainnet,
            )
            .await
            .unwrap();
        }
    }
}
