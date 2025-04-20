use web3::transports::Http;
use web3::{
    contract::Contract,
    signing::SecretKey,
    types::{Address, H160},
};
use super::constants::{ ARBITRUM_MAINNET_TOKEN_MESSENGER, ARBITRUM_SEPOLIA_TOKEN_MESSENGER, AVALANCE_FUJI_TESTNET_RPC_URL, AVALANCHE_FUJI_CONTRACT, AVALANCHE_FUJI_TOKEN_MESSENGER, AVALANCHE_MAINNET_CONTRACT, AVALANCHE_MAINNET_RPC_URL, AVALANCHE_MAINNET_TOKEN_MESSENGER, BASE_MAINNET_TOKEN_MESSENGER, BASE_SEPOLIA_TOKEN_MESSENGER, ETHEREUM_MAINNET_TOKEN_MESSENGER, ETHEREUM_SEPOLIA_TOKEN_MESSENGER, OPTIMISM_MAINNET_TOKEN_MESSENGER, OPTIMISM_SEPOLIA_TOKEN_MESSENGER, POLYGON_AMOY_TOKEN_MESSENGER, POLYGON_MAINNET_TOKEN_MESSENGER, UNICHAIN_MAINNET_TOKEN_MESSENGER, UNICHAIN_SEPOLIA_TOKEN_MESSENGER};
// use web3::types::TransactionParameters;
// use web3::ethabi::Token;
use super::{
    super::solana::constants::env,
    constants::{
        ARB_SEPOLIA_TESTNET_RPC_URL, ARBITRUM_MAINNET_CONTRACT, ARBITRUM_MAINNET_RPC_URL,
        ARBITRUM_SEPOLIA_CONTRACT, BASE_MAINNET_CONTRACT, BASE_MAINNET_RPC_URL,
        BASE_SEPOLIA_CONTRACT, BASE_TESTNET_RPC_URL, ETHEREUM_MAINNET_CONTRACT,
        ETHEREUM_MAINNET_RPC_URL, ETHEREUM_SEPOLIA_CONTRACT, ETHEREUM_SEPOLIA_TESTNET_RPC_URL,
        EVMDestinationDomain, OP_MAINNET_CONTRACT, OP_MAINNET_RPC_URL, OP_SEPOLIA_CONTRACT,
        OPTIMISM_SEPOLIA_TESTNET_RPC, POLYGON_MAINNET_RPC_URL, POLYGON_POS_AMOY_CONTRACT,
        POLYGON_POS_MAINNET_CONTRACT, POLYGON_TESTNET_RPC, UNICHAIN_CONTRACT_ADDRESS,
        UNICHAIN_MAINNET_CONTRACT, UNICHAIN_MAINNET_RPC_URL, UNICHAIN_TESTNET_RPC_URL,
    },
};
use web3::Web3;
// use super::super::solana::constants::DestinationDomain;
use anyhow::Result;
use anyhow::anyhow;
// use once_cell::sync::Lazy;

pub struct EvmManager {
    pub wallet_address: H160,
    pub wallet: SecretKey,
    pub web3: Web3<Http>,
    pub message_transmitter_contract: Contract<Http>,
    pub token_messenger_contract: Contract<Http>,
    pub message_transmitter_contract_address: H160,
    pub token_messenger_contract_address: H160, 
}

impl EvmManager {
    pub fn init(domain: u32, mainnet: bool) -> Result<Self> {
        let destination = EVMDestinationDomain::from_u32(domain)
            .ok_or_else(|| anyhow!("Invalid domain: {}", domain))?;

        let (rpc_url, message_transmitter_contract_address,token_messenger_contract_address) = match destination {
            EVMDestinationDomain::Ethereum => {
                if mainnet {
                    (ETHEREUM_MAINNET_RPC_URL, ETHEREUM_MAINNET_CONTRACT ,ETHEREUM_MAINNET_TOKEN_MESSENGER)
                } else {
                    (ETHEREUM_SEPOLIA_TESTNET_RPC_URL, ETHEREUM_SEPOLIA_CONTRACT,ETHEREUM_SEPOLIA_TOKEN_MESSENGER)
                }
            }
            EVMDestinationDomain::Avalanche => {
                if mainnet {
                    (AVALANCHE_MAINNET_RPC_URL, AVALANCHE_MAINNET_CONTRACT , AVALANCHE_MAINNET_TOKEN_MESSENGER)
                } else {
                    (AVALANCE_FUJI_TESTNET_RPC_URL, AVALANCHE_FUJI_CONTRACT,AVALANCHE_FUJI_TOKEN_MESSENGER)
                }
            }
            EVMDestinationDomain::Optimism => {
                if mainnet {
                    (OP_MAINNET_RPC_URL, OP_MAINNET_CONTRACT,OPTIMISM_MAINNET_TOKEN_MESSENGER)
                } else {
                    (OPTIMISM_SEPOLIA_TESTNET_RPC, OP_SEPOLIA_CONTRACT,OPTIMISM_SEPOLIA_TOKEN_MESSENGER)
                }
            }
            EVMDestinationDomain::Arbitrum => {
                if mainnet {
                    (ARBITRUM_MAINNET_RPC_URL, ARBITRUM_MAINNET_CONTRACT,ARBITRUM_MAINNET_TOKEN_MESSENGER)
                } else {
                    (ARB_SEPOLIA_TESTNET_RPC_URL, ARBITRUM_SEPOLIA_CONTRACT,ARBITRUM_SEPOLIA_TOKEN_MESSENGER)
                }
            }
            EVMDestinationDomain::Base => {
                if mainnet {
                    (BASE_MAINNET_RPC_URL, BASE_MAINNET_CONTRACT,BASE_MAINNET_TOKEN_MESSENGER)
                } else {
                    (BASE_TESTNET_RPC_URL, BASE_SEPOLIA_CONTRACT,BASE_SEPOLIA_TOKEN_MESSENGER)
                }
            }
            EVMDestinationDomain::PolygonPos => {
                if mainnet {
                    (POLYGON_MAINNET_RPC_URL, POLYGON_POS_MAINNET_CONTRACT,POLYGON_MAINNET_TOKEN_MESSENGER)
                } else {
                    (POLYGON_TESTNET_RPC, POLYGON_POS_AMOY_CONTRACT,POLYGON_AMOY_TOKEN_MESSENGER)
                }
            }
            EVMDestinationDomain::Unichain => {
                if mainnet {
                    (UNICHAIN_MAINNET_RPC_URL, UNICHAIN_MAINNET_CONTRACT,UNICHAIN_MAINNET_TOKEN_MESSENGER)
                } else {
                    (UNICHAIN_TESTNET_RPC_URL, UNICHAIN_CONTRACT_ADDRESS,UNICHAIN_SEPOLIA_TOKEN_MESSENGER)
                }
            }
        };

        let pk = env("FEE_PAYER_KEY");
        let pk_address = env("FEE_PAYER_ADDRESS");
        let wallet_address = hex::decode(pk_address.trim_start_matches("0x"))
            .map_err(|e| web3::Error::Decoder(format!("Invalid address format: {}", e)))?;

        let http = Http::new(rpc_url).map_err(|e| anyhow!("RPC connection failed: {}", e))?;
        let web3 = Web3::new(http);

        let wallet_address = Address::from_slice(&wallet_address);
        let message_transmitter_contract_address = message_transmitter_contract_address.parse::<Address>().unwrap();
        let token_messenger_contract_address = token_messenger_contract_address.parse::<Address>().unwrap();
        let contract = Contract::from_json(
            web3.eth(),
            message_transmitter_contract_address,
            include_bytes!("../evm/MessageTransmitter.json"),
        )?;
        let token_messenger_contract = Contract::from_json(
            web3.eth(),
            token_messenger_contract_address,
            include_bytes!("../evm/TokenMessenger.json"),
        )?;

        let private_key =
            hex::decode(pk.trim_start_matches("0x")).expect("Invalid private key hex format");
        let wallet = SecretKey::from_slice(&private_key).expect("Failed to load private key");

        Ok(Self {
            wallet_address,
            wallet,
            web3,
            message_transmitter_contract: contract,
            token_messenger_contract :token_messenger_contract,
            message_transmitter_contract_address,
            token_messenger_contract_address
        })
    }
}
