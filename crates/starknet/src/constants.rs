// use starknet_types::felt::Felt;
use starknet_in_rust::definitions::block_context::StarknetChainId;

pub(crate) const CAIRO_0_ACCOUNT_CONTRACT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/accounts_artifacts/OpenZeppelin/0.5.1/Account.cairo/Account.json"
);

pub const CAIRO_0_ACCOUNT_CONTRACT_HASH: &str =
    "0x4d07e40e93398ed3c76981e72dd1fd22557a78ce36c0515f679e27f0bb5bc5f";

pub(crate) const ERC20_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/accounts_artifacts/ERC20_Mintable_OZ_0.2.0.json");

pub(crate) const ERC20_CONTRACT_CLASS_HASH: &str =
    "0x6A22BF63C7BC07EFFA39A25DFBD21523D211DB0100A0AFD054D172B81840EAF";

pub const ERC20_CONTRACT_ADDRESS: &str =
    "0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7";

pub(crate) const UDC_CONTRACT_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/accounts_artifacts/UDC_OZ_0.5.0.json");

pub(crate) const UDC_CONTRACT_CLASS_HASH: &str =
    "0x7B3E05F48F0C69E4A65CE5E076A66271A527AFF2C34CE1083EC6E1526997A69";

pub(crate) const UDC_CONTRACT_ADDRESS: &str =
    "0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF";

pub const DEVNET_DEFAULT_TEST_SEED: u32 = 123;
pub const DEVNET_DEFAULT_TOTAL_ACCOUNTS: u8 = 10;
pub const DEVNET_DEFAULT_INITIAL_BALANCE: u128 = 1000000000000000000000;
pub const DEVNET_DEFAULT_GAS_PRICE: u64 = 100_000_000_000;
pub const DEVNET_DEFAULT_HOST: &str = "127.0.0.1";
pub const DEVNET_DEFAULT_PORT: u16 = 5050;
pub const DEVNET_DEFAULT_TIMEOUT: u16 = 120;
pub const DEVNET_DEFAULT_CHAIN_ID: StarknetChainId = StarknetChainId::TestNet;
