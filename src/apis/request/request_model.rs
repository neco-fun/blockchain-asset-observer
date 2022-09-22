use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetERC20BalanceRequest {
    pub network: u8,
    pub contract_type: String,
    pub public_address: String,
}

#[derive(Debug, Deserialize)]
pub struct GetNFTOwnershipRequest {
    pub network: u8,
    pub game_client: u8,
    pub public_address: String,
}