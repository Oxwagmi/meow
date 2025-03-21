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
    Bridge {
        #[arg(long,short,default_value = "false")]
        mainnet: bool ,
        #[arg(long)]
        to_chain: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
    },
}
