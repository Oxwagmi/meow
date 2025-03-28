//https://developers.circle.com/stablecoins/evm-smart-contracts
// Testnet Contract Addresses
pub const UNICHAIN_CONTRACT_ADDRESS: &str = "0xbc498c326533d675cf571B90A2Ced265ACb7d086";
pub const ETHEREUM_SEPOLIA_CONTRACT: &str = "0x7865fAfC2db2093669d92c0F33AeEF291086BEFD";
pub const AVALANCHE_FUJI_CONTRACT: &str = "0xa9fb1b3009dcb79e2fe346c16a604b8fa8ae0a79";
pub const OP_SEPOLIA_CONTRACT: &str = "0x7865fAfC2db2093669d92c0F33AeEF291086BEFD";
pub const ARBITRUM_SEPOLIA_CONTRACT: &str = "0xaCF1ceeF35caAc005e15888dDb8A3515C41B4872";
pub const BASE_SEPOLIA_CONTRACT: &str = "0x7865fAfC2db2093669d92c0F33AeEF291086BEFD";
pub const POLYGON_POS_AMOY_CONTRACT: &str = "0x7865fAfC2db2093669d92c0F33AeEF291086BEFD";
pub const UNICHAIN_SEPOLIA_CONTRACT: &str = "0xbc498c326533d675cf571B90A2Ced265ACb7d086";

// Testnet RPC URLs
pub const UNICHAIN_TESTNET_RPC_URL: &str = "https://unichain-sepolia.api.onfinality.io/public";
pub const BASE_TESTNET_RPC_URL: &str = "https://sepolia.base.org";
pub const ETHEREUM_SEPOLIA_TESTNET_RPC_URL: &str = "https://sepolia-rpc.publicnode.com";
pub const ARB_SEPOLIA_TESTNET_RPC_URL: &str = "https://arbitrum-sepolia-rpc.publicnode.com";
pub const OPTIMISM_SEPOLIA_TESTNET_RPC: &str = "https://sepolia.optimism.io/";
pub const POLYGON_TESTNET_RPC: &str = "https://endpoints.omniatech.io/v1/matic/mumbai/public";
pub const AVALANCE_FUJI_TESTNET_RPC_URL: &str = "https://avalanche-fuji-c-chain-rpc.publicnode.com";

// Mainnet Contract Addresses
pub const ETHEREUM_MAINNET_CONTRACT: &str = "0x0a992d191deec32afe36203ad87d7d289a738f81";
pub const AVALANCHE_MAINNET_CONTRACT: &str = "0x8186359af5f57fbb40c6b14a588d2a59c0c29880";
pub const OP_MAINNET_CONTRACT: &str = "0x4d41f22c5a0e5c74090899e5a8fb597a8842b3e8";
pub const ARBITRUM_MAINNET_CONTRACT: &str = "0xC30362313FBBA5cf9163F0bb16a0e01f01A896ca";
pub const BASE_MAINNET_CONTRACT: &str = "0xAD09780d193884d503182aD4588450C416D6F9D4";
pub const POLYGON_POS_MAINNET_CONTRACT: &str = "0xF3be9355363857F3e001be68856A2f96b4C39Ba9";
pub const UNICHAIN_MAINNET_CONTRACT: &str = "0x353bE9E2E38AB1D19104534e4edC21c643Df86f4";

// Mainnet RPC URLs
pub const ETHEREUM_MAINNET_RPC_URL: &str = "https://ethereum-rpc.publicnode.com";
pub const AVALANCHE_MAINNET_RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";
pub const OP_MAINNET_RPC_URL: &str = "https://mainnet.optimism.io/";
pub const ARBITRUM_MAINNET_RPC_URL: &str = "https://arb1.arbitrum.io/rpc";
pub const BASE_MAINNET_RPC_URL: &str = "https://mainnet.base.org";
pub const POLYGON_MAINNET_RPC_URL: &str = "https://polygon-rpc.com/";
pub const UNICHAIN_MAINNET_RPC_URL: &str = "	https://mainnet.unichain.org";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EVMDestinationDomain {
    Ethereum = 0,
    Avalanche = 1,
    Optimism = 2,
    Arbitrum = 3,
    Base = 6,
    PolygonPos = 7,
    Unichain = 10,
}

impl EVMDestinationDomain {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Ethereum),
            1 => Some(Self::Avalanche),
            2 => Some(Self::Optimism),
            3 => Some(Self::Arbitrum),
            6 => Some(Self::Base),
            7 => Some(Self::PolygonPos),
            10 => Some(Self::Unichain),
            _ => None,
        }
    }
}
