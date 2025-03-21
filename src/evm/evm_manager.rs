use web3::{
    contract::Contract, signing::SecretKey, types::{Address, H160}};
use web3::transports::Http;
// use web3::types::TransactionParameters;
// use web3::ethabi::Token;
use web3::Web3;
use super::{super::solana::constants::env, constants::{EVMDestinationDomain, ARBITRUM_MAINNET_CONTRACT, ARBITRUM_MAINNET_RPC_URL, ARBITRUM_SEPOLIA_CONTRACT, ARB_SEPOLIA_TESTNET_RPC_URL, BASE_MAINNET_CONTRACT, BASE_MAINNET_RPC_URL, BASE_SEPOLIA_CONTRACT, BASE_TESTNET_RPC_URL, ETHEREUM_MAINNET_CONTRACT, ETHEREUM_MAINNET_RPC_URL, ETHEREUM_SEPOLIA_CONTRACT, ETHEREUM_SEPOLIA_TESTNET_RPC_URL, OPTIMISM_SEPOLIA_TESTNET_RPC, OP_MAINNET_CONTRACT, OP_MAINNET_RPC_URL, OP_SEPOLIA_CONTRACT, POLYGON_MAINNET_RPC_URL, POLYGON_POS_AMOY_CONTRACT, POLYGON_POS_MAINNET_CONTRACT, POLYGON_TESTNET_RPC, UNICHAIN_CONTRACT_ADDRESS, UNICHAIN_MAINNET_CONTRACT, UNICHAIN_MAINNET_RPC_URL, UNICHAIN_TESTNET_RPC_URL}};
// use super::super::solana::constants::DestinationDomain;
use anyhow::Result;
use anyhow::anyhow;
// use once_cell::sync::Lazy;



pub struct  EvmManager {
    pub wallet_address: H160,
    pub wallet: SecretKey,
    pub web3: Web3<Http>,
    pub message_transmitter_contract: Contract<Http>,
pub contract_address: H160,
}



impl EvmManager {
    pub fn init(domain: u32, mainnet: bool) -> Result<Self> {
        let destination = EVMDestinationDomain::from_u32(domain)
            .ok_or_else(|| anyhow!("Invalid domain: {}", domain))?;

        let (rpc_url, contract_address) = match destination {
            EVMDestinationDomain::Ethereum => {
                if mainnet {
                    (ETHEREUM_MAINNET_RPC_URL, ETHEREUM_MAINNET_CONTRACT)
                } else {
                    (ETHEREUM_SEPOLIA_TESTNET_RPC_URL, ETHEREUM_SEPOLIA_CONTRACT)
                }
            }
            EVMDestinationDomain::Optimism => {
                if mainnet {
                    (OP_MAINNET_RPC_URL, OP_MAINNET_CONTRACT)
                } else {
                    (OPTIMISM_SEPOLIA_TESTNET_RPC, OP_SEPOLIA_CONTRACT)
                }
            }
            EVMDestinationDomain::Arbitrum => {
                if mainnet {
                    (ARBITRUM_MAINNET_RPC_URL, ARBITRUM_MAINNET_CONTRACT)
                } else {
                    (ARB_SEPOLIA_TESTNET_RPC_URL, ARBITRUM_SEPOLIA_CONTRACT)
                }
            }
            EVMDestinationDomain::Base => {
                if mainnet {
                    (BASE_MAINNET_RPC_URL, BASE_MAINNET_CONTRACT)
                } else {
                    (BASE_TESTNET_RPC_URL, BASE_SEPOLIA_CONTRACT)
                }
            }
            EVMDestinationDomain::PolygonPos => {
                if mainnet {
                    (POLYGON_MAINNET_RPC_URL, POLYGON_POS_MAINNET_CONTRACT)
                } else {
                    (POLYGON_TESTNET_RPC, POLYGON_POS_AMOY_CONTRACT)
                }
            }
            EVMDestinationDomain::Unichain => {
                if mainnet {
                    (UNICHAIN_MAINNET_RPC_URL, UNICHAIN_MAINNET_CONTRACT)
                } else {
                    (UNICHAIN_TESTNET_RPC_URL, UNICHAIN_CONTRACT_ADDRESS)
                }
            }
        };

        let pk = env("FEE_PAYER_KEY");
        let pk_address = env("FEE_PAYER_ADDRESS");
        let wallet_address = hex::decode(pk_address.trim_start_matches("0x"))
            .map_err(|e| web3::Error::Decoder(format!("Invalid address format: {}", e)))?;

        let http = Http::new(rpc_url)
            .map_err(|e| anyhow!("RPC connection failed: {}", e))?;
        let web3 = Web3::new(http);

        let wallet_address = Address::from_slice(&wallet_address);
        let contract_address = contract_address.parse::<Address>().unwrap();
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("../evm/MessageTransmitter.json"),
        )?;

        let private_key = hex::decode(pk.trim_start_matches("0x"))
            .expect("Invalid private key hex format");
        let wallet = SecretKey::from_slice(&private_key)
            .expect("Failed to load private key");

        Ok(Self {
            wallet_address,
            wallet,
            web3,
            message_transmitter_contract: contract,
            contract_address,
        })
    }
}


