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

// Mainnet Contract Addresses
pub const ETHEREUM_MAINNET_CONTRACT: &str = "0xb2f38107a18f8599331677c14374fd3a952fb2c8";
pub const AVALANCHE_MAINNET_CONTRACT: &str = "0x21f337db7a718f23e061262470af8c1fd01232d1";
pub const OP_MAINNET_CONTRACT: &str = "0xdb2831eaf163be1b564d437a97372deb0046c70d";
pub const ARBITRUM_MAINNET_CONTRACT: &str = "0xe189bdcfbcecec917b937247666a44ed959d81e4";
pub const BASE_MAINNET_CONTRACT: &str = "0x827ae40E55C4355049ab91e441b6e269e4091441";
pub const POLYGON_POS_MAINNET_CONTRACT: &str = "0x02d9fa3e7f870E5FAA7Ca6c112031E0ddC5E646C";
pub const UNICHAIN_MAINNET_CONTRACT: &str = "0x395b1be6E432033B676e3e36B2c2121a1f952622";

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
    // Avalanche = 1,
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
            // 1 => Some(Self::Avalanche),
            2 => Some(Self::Optimism),
            3 => Some(Self::Arbitrum),
            6 => Some(Self::Base),
            7 => Some(Self::PolygonPos),
            10 => Some(Self::Unichain),
            _ => None,
        }
    }
}
