use clap::Parser;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(flatten)]
    pub args: Args,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {}

#[derive(Debug, Parser)]
pub enum Command {
    BridgeSolanaUSDC {
        #[arg(long, short, default_value = "false")]
        mainnet: bool,
        #[arg(long, default_value = "false")]
        safe_format_usdc: bool,
        #[arg(long)]
        to_chain: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
    },
    BridgeEvmUSDC {
        #[arg(long, short, default_value = "false")]
        mainnet: bool,
        // #[arg(long, default_value = "false")]
        // safe_format_usdc: bool,
        #[arg(long)]
        from_chain: String,
        // #[arg(long)]
        // to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long, default_value = "10")]
        retry_secs: u64,
    },
    MannualRedeemUsdc {
        #[arg(long, short, default_value = "false")]
        mainnet: bool,
        #[arg(long)]
        txn_hash: String,
        #[arg(long)]
        remote_domain: u32,
        #[arg(long, default_value = "")]
        remote_usdc: String,
        #[arg(long, default_value = "10")]
        retry_secs: u64,
    },
}
