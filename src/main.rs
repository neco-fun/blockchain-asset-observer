use std::fs;
use std::net::SocketAddr;

use ethers::providers::{Http, Provider};
use log::{info, LevelFilter};
use once_cell::sync::OnceCell;

use common::defines::NetworkType;
use common::provider::ProviderManager;

use crate::common::defines::{Error, BSC_MAIN_NETWORK_RPC, BSC_TEST_NETWORK_RPC};
use crate::models::{Address, AddressConfig};

mod apis;
mod common;
mod models;
mod router;
mod services;

static ADDRESS_MANAGER: OnceCell<AddressConfig> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let address_content = fs::read_to_string("address.toml")?;
    let address: AddressConfig = toml::from_str(&address_content)?;
    ADDRESS_MANAGER.set(address).unwrap();

    let bsc_main_client =
        Provider::<Http>::try_from(BSC_MAIN_NETWORK_RPC).expect("get bsc mainnet provider failed.");
    let bsc_test_client =
        Provider::<Http>::try_from(BSC_TEST_NETWORK_RPC).expect("get bsc testnet provider failed.");

    ProviderManager::instance().set_provider(NetworkType::BSCMainNetwork, bsc_main_client);
    ProviderManager::instance().set_provider(NetworkType::BSCTestNetwork, bsc_test_client);

    let app = router::new_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("web server is listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
