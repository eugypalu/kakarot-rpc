use rstest::fixture;
use tracing_subscriber::{filter, FmtSubscriber};
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
use {
    super::katana::Katana,
    super::mongo::RANDOM_BYTES_SIZE,
    crate::test_utils::evm_contract::KakarotEvmContract,
    alloy_dyn_abi::DynSolValue,
    reth_primitives::{Address, U256},
};

/// This fixture deploys a counter contract on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
#[awt]
pub async fn counter(#[future] katana_empty: Katana) -> (Katana, KakarotEvmContract) {
    let eoa = katana_empty.eoa();
    let contract = eoa.deploy_evm_contract(Some("Counter"), &[]).await.expect("Failed to deploy Counter contract");
    (katana_empty, contract)
}

/// This fixture deploys an empty contract on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
#[awt]
pub async fn contract_empty(#[future] katana_empty: Katana) -> (Katana, KakarotEvmContract) {
    let eoa = katana_empty.eoa();
    let contract = eoa.deploy_evm_contract(None, &[]).await.expect("Failed to deploy empty contract");
    (katana_empty, contract)
}

/// This fixture deploys an ERC20 contract on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
#[awt]
pub async fn erc20(#[future] katana_empty: Katana) -> (Katana, KakarotEvmContract) {
    let eoa = katana_empty.eoa();

    let contract = eoa
        .deploy_evm_contract(
            Some("ERC20"),
            &[
                DynSolValue::String("Test".into()),   // name
                DynSolValue::String("TT".into()),     // symbol
                DynSolValue::Uint(U256::from(18), 8), // decimals
            ],
        )
        .await
        .expect("Failed to deploy ERC20 contract");
    (katana_empty, contract)
}

/// This fixture deploys the plain opcodes contract on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
#[awt]
pub async fn plain_opcodes(#[future] counter: (Katana, KakarotEvmContract)) -> (Katana, KakarotEvmContract) {
    let katana = counter.0;
    let counter = counter.1;
    let eoa = katana.eoa();
    let counter_address = Address::from_slice(&counter.evm_address.to_bytes_be()[12..]);
    let contract = eoa
        .deploy_evm_contract(
            Some("PlainOpcodes"),
            &[
                DynSolValue::Address(counter_address), // counter address
            ],
        )
        .await
        .expect("Failed to deploy PlainOpcodes contract");
    (katana, contract)
}

/// This fixture creates a new test environment on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
pub async fn katana() -> Katana {
    // Create a new test environment on Katana
    Katana::new(RANDOM_BYTES_SIZE).await
}

/// This fixture creates a new test environment on Katana.
#[cfg(any(test, feature = "arbitrary", feature = "testing"))]
#[fixture]
pub async fn katana_empty() -> Katana {
    // Create a new test environment on Katana
    Katana::new_empty().await
}

/// This fixture configures the tests. The following setup
/// is used:
/// - The log level is set to `info`
#[fixture]
pub fn setup() {
    let filter = filter::EnvFilter::new("info");
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}
