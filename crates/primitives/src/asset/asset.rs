use typeshare::typeshare;

use crate::{asset::AssetId, AssetType, Chain};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[typeshare(swift = "Equatable, Hashable, Sendable")]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: AssetId,
    #[typeshare(skip)]
    pub chain: Chain,
    #[typeshare(skip)]
    pub token_id: Option<String>,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    #[serde(rename = "type")]
    pub asset_type: AssetType,
}

impl Chain {
    pub fn new_asset(
        &self,
        name: String,
        symbol: String,
        decimals: i32,
        asset_type: AssetType,
    ) -> Asset {
        Asset {
            id: self.as_asset_id(),
            chain: *self,
            token_id: None,
            name,
            symbol,
            decimals,
            asset_type,
        }
    }
}

impl Asset {
    pub fn new(
        id: AssetId,
        name: String,
        symbol: String,
        decimals: i32,
        asset_type: AssetType,
    ) -> Asset {
        Asset {
            id: id.clone(),
            chain: id.chain,
            name,
            symbol,
            decimals,
            asset_type,
            token_id: id.token_id.clone(),
        }
    }

    pub fn from_chain(chain: Chain) -> Asset {
        match chain {
            Chain::Bitcoin => chain.new_asset(
                "Bitcoin".to_string(),
                "BTC".to_string(),
                8,
                AssetType::NATIVE,
            ),
            Chain::Ethereum => chain.new_asset(
                "Ethereum".to_string(),
                "ETH".to_string(),
                18,
                AssetType::NATIVE,
            ),
            Chain::Solana => chain.new_asset(
                "Solana".to_string(),
                "SOL".to_string(),
                9,
                AssetType::NATIVE,
            ),
        }
    }
}
